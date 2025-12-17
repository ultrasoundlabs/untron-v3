use poseidon::poseidon2;
use sha256::sha256_var;
use std::ecdsa_secp256k1;

global MAX_BLOCKS: u32 = 1024;
global SR_COUNT: u32 = 27;

// Structure representing the necessary data for each block in the proof range
struct Block {
    prev_block_id: [u8; 32],
    new_block_id: [u8; 32],
    tx_root: [u8; 32],
    raw_data: [u8; 128],
    raw_data_length: u64,
    public_key: [u8; 64],
    signature: [u8; 64],
}

// ----------------- small helpers -----------------

fn concat_32_32(a: [u8; 32], b: [u8; 32]) -> [u8; 64] {
    let mut out: [u8; 64] = [0; 64];
    for i in 0..32 {
        out[i] = a[i];
        out[32 + i] = b[i];
    }
    out
}

fn take_32_from_64(src: [u8; 64], start: u32) -> [u8; 32] {
    let mut out: [u8; 32] = [0; 32];
    for i in 0..32 {
        out[i] = src[start + i];
    }
    out
}

fn b2u32(b: bool) -> u32 {
    u32::from(b)
}

// NOTE: We intentionally do NOT derive addresses from signing keys in-circuit.
// Proposer uniqueness is checked using the SR address provided in the header's `witness_address` field.

// ----------------- varint parser -----------------
fn read_varint_u64(bytes: [u8; 128], offset: u32, active: bool) -> (u64, u32) {
    // keep the bound check unconditional to avoid OOB in bytes[offset+i]
    assert(offset <= 118u32);

    let mut value: u64 = 0;
    let mut consumed: u32 = 0;
    let mut done: bool = false;

    for i in 0..10 {
        // only take bytes when active
        let take: bool = active & (!done);

        let b: u8 = bytes[offset + i];

        if take {
            let low7: u64 = (b & 0x7fu8) as u64;
            let shift: u64 = 7u64 * (i as u64);
            value = value | (low7 << shift);
            consumed += 1u32;

            let is_last: bool = (b & 0x80u8) == 0u8;
            done = done | is_last;
        }
    }

    // only require termination when active
    assert((!active) | done);

    (value, offset + consumed)
}

// ----------------- verification helpers -----------------

fn verify_proposer(
    active: bool,
    proposer_sr_addr: [u8; 20],
    srs: [[u8; 20]; 27],
    used: &mut [bool; 27],
) -> u32 {
    let mut found: bool = false;
    let mut idx: u32 = 0;

    for i in 0..27 {
        let is_owner: bool = proposer_sr_addr == srs[i];
        let take: bool = active & (!found) & is_owner;

        assert((!take) | (!used[i]));
        used[i] = used[i] | take;

        found = found | take;
        idx = idx + i * b2u32(take);
    }

    assert((!active) | found);
    idx
}

fn parse_block_header(block: Block, active: bool) -> (u64, [u8; 20]) {
    let bytes: [u8; 128] = block.raw_data;
    let mut offset: u32 = 0;
    let a: u32 = b2u32(active);

    let mut witness_sr_addr: [u8; 20] = [0; 20];

    // Field 1: timestamp
    let key: u8 = bytes[offset];
    assert((!active) | ((key >> 3) == 1));
    assert((!active) | ((key & 0x07) == 0));
    offset += a;

    let (timestamp_ms, off2) = read_varint_u64(bytes, offset, active);
    offset = off2;

    // Field 2: txTrieRoot
    {
        let key2: u8 = bytes[offset];
        assert((!active) | ((key2 >> 3) == 2));
        assert((!active) | ((key2 & 0x07) == 2));
        offset += a;

        let tx_len: u8 = bytes[offset];
        assert((!active) | (tx_len == 32));
        offset += a;

        for i in 0..32 {
            assert((!active) | (block.tx_root[i] == bytes[offset]));
            offset += a;
        }
    }

    // Field 3: parentHash
    {
        let key3: u8 = bytes[offset];
        assert((!active) | ((key3 >> 3) == 3));
        assert((!active) | ((key3 & 0x07) == 2));
        offset += a;

        let parent_len: u8 = bytes[offset];
        assert((!active) | (parent_len == 32));
        offset += a;

        for i in 0..32 {
            assert((!active) | (block.prev_block_id[i] == bytes[offset]));
            offset += a;
        }
    }

    // Field 7: block number
    {
        let key7: u8 = bytes[offset];
        assert((!active) | ((key7 >> 3) == 7));
        assert((!active) | ((key7 & 0x07) == 0));
        offset += a;

        let (block_number, off3) = read_varint_u64(bytes, offset, active);
        offset = off3;

        let mut encoded_number: u64 = 0;
        for j in 0..8 {
            encoded_number = (encoded_number << 8) | (block.new_block_id[j] as u64);
        }
        assert((!active) | (block_number == encoded_number));
    }

    // Field 9: witness_address
    {
        let key9: u8 = bytes[offset];
        assert((!active) | ((key9 >> 3) == 9));
        assert((!active) | ((key9 & 0x07) == 2));
        offset += a;

        let wlen: u8 = bytes[offset];
        assert((!active) | (wlen == 21));
        offset += a;

        assert((!active) | (bytes[offset] == 0x41));
        offset += a;

        for j in 0..20 {
            let b: u8 = bytes[offset];
            // only store the address when active
            witness_sr_addr[j] = if active { b } else { 0u8 };
            offset += a;
        }
    }

    // Field 10: version
    {
        let key10: u8 = bytes[offset];
        assert((!active) | ((key10 >> 3) == 10));
        assert((!active) | ((key10 & 0x07) == 0));
        offset += a;

        let (version, off4) = read_varint_u64(bytes, offset, active);
        offset = off4;
        assert((!active) | (version == 32));
    }

    // only enforce length equality when active
    assert((!active) | ((offset as u64) == block.raw_data_length));

    (timestamp_ms, witness_sr_addr)
}

fn verify_signature(active: bool, pub_key: [u8; 64], signature: [u8; 64], message_hash: [u8; 32]) {
    let pub_key_x: [u8; 32] = take_32_from_64(pub_key, 0);
    let pub_key_y: [u8; 32] = take_32_from_64(pub_key, 32);

    let ok: bool = ecdsa_secp256k1::verify_signature(pub_key_x, pub_key_y, signature, message_hash);
    assert((!active) | ok);
}

fn field_from_u8_20_be(x: [u8; 20]) -> Field {
    let mut padded: [u8; 32] = [0u8; 32];
    // left-pad for big-endian interpretation: put the 20 bytes in the last 20 slots
    for i in 0..20 {
        padded[12 + i] = x[i];
    }
    Field::from_be_bytes(padded) // wraps if needed
}

fn field_from_u8_32_be(x: [u8; 32]) -> Field {
    Field::from_be_bytes(x) // wraps if needed
}

// Computes the Poseidon2-based hash of SR data.
pub fn compute_sr_data_hash(srs: [[u8; 20]; 27], witness_delegatees: [[[u8; 32]; 2]; 27]) -> Field {
    let mut inputs: [Field; 81] = [0; 81];

    for i in 0..27 {
        // first object
        inputs[i] = field_from_u8_20_be(srs[i]);

        // second object (two 32-byte items per i)
        let j = 27 + 2 * i;
        inputs[j] = field_from_u8_32_be(witness_delegatees[i][0]);
        inputs[j + 1] = field_from_u8_32_be(witness_delegatees[i][1]);
    }

    poseidon2::Poseidon2::hash(inputs, 81)
}

// ----------------- main -----------------

fn main(
    sr_data_hash: pub Field,
    starting_block: pub [u8; 32],
    ending_block: pub [u8; 32],
    ending_block_tx_trie_root: pub [u8; 32],
    ending_block_timestamp: pub u32,
    num_blocks: pub u32,
    srs: [[u8; 20]; 27],
    witness_delegate_pubkeys: [[[u8; 32]; 2]; 27],
    blocks: [Block; MAX_BLOCKS],
) {
    // bounds
    assert(num_blocks > 0u32);
    assert(num_blocks <= MAX_BLOCKS);
    assert(num_blocks <= SR_COUNT); // because proposer uniqueness uses [bool; 27]

    // SR data hash check
    let computed_sr_hash: Field = compute_sr_data_hash(srs, witness_delegate_pubkeys);
    assert(computed_sr_hash == sr_data_hash);

    // Anchor start (block 0 must exist since num_blocks > 0)
    assert(blocks[0].prev_block_id == starting_block);

    // Anchor end via dynamic index (allowed; costs a bit)
    let end_index: u32 = num_blocks - 1u32;
    assert(blocks[end_index].new_block_id == ending_block);
    assert(blocks[end_index].tx_root == ending_block_tx_trie_root);

    let mut used_recent: [bool; 27] = [false; 27];

    for i in 0..MAX_BLOCKS {
        let active: bool = i < num_blocks;
        let block: Block = blocks[i];

        // parse header only constrains when active
        let (timestamp_ms, witness_sr_addr) = parse_block_header(block, active);

        // hash with length masked to 0 when inactive
        assert((!active) | (block.raw_data_length <= 128u64));
        let len_u32: u32 = if active {
            block.raw_data_length as u32
        } else {
            0u32
        };
        let block_hash: [u8; 32] = sha256_var(block.raw_data, len_u32);

        // id tail matches hash tail (only when active)
        for k in 8..32 {
            assert((!active) | (block.new_block_id[k] == block_hash[k]));
        }

        // signature check (only when active)
        verify_signature(active, block.public_key, block.signature, block_hash);

        // proposer uniqueness only for active blocks
        let proposer_index: u32 = verify_proposer(active, witness_sr_addr, srs, &mut used_recent);

        // pubkey binding only for active blocks
        let expected_pubkey: [u8; 64] = concat_32_32(
            witness_delegate_pubkeys[proposer_index][0],
            witness_delegate_pubkeys[proposer_index][1],
        );
        assert((!active) | (block.public_key == expected_pubkey));

        // last active block timestamp check
        let is_last_active: bool = active & ((i + 1u32) == num_blocks);
        let ms_mod: u64 = timestamp_ms % 1000u64;
        assert((!is_last_active) | (ms_mod == 0u64));

        let timestamp_sec: u32 = (timestamp_ms / 1000u64) as u32;
        assert((!is_last_active) | (timestamp_sec == ending_block_timestamp));
    }
}
