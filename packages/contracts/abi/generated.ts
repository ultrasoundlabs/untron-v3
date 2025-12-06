//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create2Utils
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const create2UtilsAbi = [
  {
    type: 'constructor',
    inputs: [{ name: 'create2Prefix', internalType: 'bytes1', type: 'bytes1' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'salt', internalType: 'bytes32', type: 'bytes32' }],
    name: 'deployReceiver',
    outputs: [
      { name: 'receiver', internalType: 'address payable', type: 'address' },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'controller', internalType: 'address', type: 'address' },
      { name: 'salt', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'predictReceiverAddress',
    outputs: [{ name: 'predicted', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'receiverBytecode',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'pure',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ECDSA
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ecdsaAbi = [
  { type: 'error', inputs: [], name: 'ECDSAInvalidSignature' },
  {
    type: 'error',
    inputs: [{ name: 'length', internalType: 'uint256', type: 'uint256' }],
    name: 'ECDSAInvalidSignatureLength',
  },
  {
    type: 'error',
    inputs: [{ name: 's', internalType: 'bytes32', type: 'bytes32' }],
    name: 'ECDSAInvalidSignatureS',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC20
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc20Abi = [
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'error',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'allowance', internalType: 'uint256', type: 'uint256' },
      { name: 'needed', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'ERC20InsufficientAllowance',
  },
  {
    type: 'error',
    inputs: [
      { name: 'sender', internalType: 'address', type: 'address' },
      { name: 'balance', internalType: 'uint256', type: 'uint256' },
      { name: 'needed', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'ERC20InsufficientBalance',
  },
  {
    type: 'error',
    inputs: [{ name: 'approver', internalType: 'address', type: 'address' }],
    name: 'ERC20InvalidApprover',
  },
  {
    type: 'error',
    inputs: [{ name: 'receiver', internalType: 'address', type: 'address' }],
    name: 'ERC20InvalidReceiver',
  },
  {
    type: 'error',
    inputs: [{ name: 'sender', internalType: 'address', type: 'address' }],
    name: 'ERC20InvalidSender',
  },
  {
    type: 'error',
    inputs: [{ name: 'spender', internalType: 'address', type: 'address' }],
    name: 'ERC20InvalidSpender',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IBlockRangeProver
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iBlockRangeProverAbi = [
  {
    type: 'function',
    inputs: [
      { name: 'srs', internalType: 'bytes20[27]', type: 'bytes20[27]' },
      {
        name: 'witnessDelegatees',
        internalType: 'bytes20[27]',
        type: 'bytes20[27]',
      },
      { name: 'startingBlock', internalType: 'bytes32', type: 'bytes32' },
      { name: 'endingBlock', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'endingBlockTxTrieRoot',
        internalType: 'bytes32',
        type: 'bytes32',
      },
      { name: 'endingBlockTimestamp', internalType: 'uint32', type: 'uint32' },
      { name: 'zkProof', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'proveBlockRange',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IBridger
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iBridgerAbi = [
  {
    type: 'function',
    inputs: [
      { name: 'token', internalType: 'address', type: 'address' },
      { name: 'inAmount', internalType: 'uint256', type: 'uint256' },
      { name: 'payload', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'bridge',
    outputs: [{ name: 'outAmount', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'nonpayable',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC1155Errors
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc1155ErrorsAbi = [
  {
    type: 'error',
    inputs: [
      { name: 'sender', internalType: 'address', type: 'address' },
      { name: 'balance', internalType: 'uint256', type: 'uint256' },
      { name: 'needed', internalType: 'uint256', type: 'uint256' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'ERC1155InsufficientBalance',
  },
  {
    type: 'error',
    inputs: [{ name: 'approver', internalType: 'address', type: 'address' }],
    name: 'ERC1155InvalidApprover',
  },
  {
    type: 'error',
    inputs: [
      { name: 'idsLength', internalType: 'uint256', type: 'uint256' },
      { name: 'valuesLength', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'ERC1155InvalidArrayLength',
  },
  {
    type: 'error',
    inputs: [{ name: 'operator', internalType: 'address', type: 'address' }],
    name: 'ERC1155InvalidOperator',
  },
  {
    type: 'error',
    inputs: [{ name: 'receiver', internalType: 'address', type: 'address' }],
    name: 'ERC1155InvalidReceiver',
  },
  {
    type: 'error',
    inputs: [{ name: 'sender', internalType: 'address', type: 'address' }],
    name: 'ERC1155InvalidSender',
  },
  {
    type: 'error',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'owner', internalType: 'address', type: 'address' },
    ],
    name: 'ERC1155MissingApprovalForAll',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC1363
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc1363Abi = [
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approveAndCall',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'approveAndCall',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferAndCall',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'transferAndCall',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'transferFromAndCall',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFromAndCall',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC165
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc165Abi = [
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC20
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc20Abi = [
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC20Errors
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc20ErrorsAbi = [
  {
    type: 'error',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'allowance', internalType: 'uint256', type: 'uint256' },
      { name: 'needed', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'ERC20InsufficientAllowance',
  },
  {
    type: 'error',
    inputs: [
      { name: 'sender', internalType: 'address', type: 'address' },
      { name: 'balance', internalType: 'uint256', type: 'uint256' },
      { name: 'needed', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'ERC20InsufficientBalance',
  },
  {
    type: 'error',
    inputs: [{ name: 'approver', internalType: 'address', type: 'address' }],
    name: 'ERC20InvalidApprover',
  },
  {
    type: 'error',
    inputs: [{ name: 'receiver', internalType: 'address', type: 'address' }],
    name: 'ERC20InvalidReceiver',
  },
  {
    type: 'error',
    inputs: [{ name: 'sender', internalType: 'address', type: 'address' }],
    name: 'ERC20InvalidSender',
  },
  {
    type: 'error',
    inputs: [{ name: 'spender', internalType: 'address', type: 'address' }],
    name: 'ERC20InvalidSpender',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC20Metadata
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc20MetadataAbi = [
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC721Errors
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc721ErrorsAbi = [
  {
    type: 'error',
    inputs: [
      { name: 'sender', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'owner', internalType: 'address', type: 'address' },
    ],
    name: 'ERC721IncorrectOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'ERC721InsufficientApproval',
  },
  {
    type: 'error',
    inputs: [{ name: 'approver', internalType: 'address', type: 'address' }],
    name: 'ERC721InvalidApprover',
  },
  {
    type: 'error',
    inputs: [{ name: 'operator', internalType: 'address', type: 'address' }],
    name: 'ERC721InvalidOperator',
  },
  {
    type: 'error',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'ERC721InvalidOwner',
  },
  {
    type: 'error',
    inputs: [{ name: 'receiver', internalType: 'address', type: 'address' }],
    name: 'ERC721InvalidReceiver',
  },
  {
    type: 'error',
    inputs: [{ name: 'sender', internalType: 'address', type: 'address' }],
    name: 'ERC721InvalidSender',
  },
  {
    type: 'error',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ERC721NonexistentToken',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ILayerZeroEndpointV2
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iLayerZeroEndpointV2Abi = [
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
      { name: '_nonce', internalType: 'uint64', type: 'uint64' },
      { name: '_payloadHash', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'burn',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      {
        name: '_origin',
        internalType: 'struct Origin',
        type: 'tuple',
        components: [
          { name: 'srcEid', internalType: 'uint32', type: 'uint32' },
          { name: 'sender', internalType: 'bytes32', type: 'bytes32' },
          { name: 'nonce', internalType: 'uint64', type: 'uint64' },
        ],
      },
      { name: '_guid', internalType: 'bytes32', type: 'bytes32' },
      { name: '_message', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'clear',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_from', internalType: 'address', type: 'address' },
      { name: '_to', internalType: 'address', type: 'address' },
      { name: '_guid', internalType: 'bytes32', type: 'bytes32' },
      { name: '_index', internalType: 'uint16', type: 'uint16' },
    ],
    name: 'composeQueue',
    outputs: [
      { name: 'messageHash', internalType: 'bytes32', type: 'bytes32' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_eid', internalType: 'uint32', type: 'uint32' }],
    name: 'defaultReceiveLibrary',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_eid', internalType: 'uint32', type: 'uint32' }],
    name: 'defaultReceiveLibraryTimeout',
    outputs: [
      { name: 'lib', internalType: 'address', type: 'address' },
      { name: 'expiry', internalType: 'uint256', type: 'uint256' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_eid', internalType: 'uint32', type: 'uint32' }],
    name: 'defaultSendLibrary',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'eid',
    outputs: [{ name: '', internalType: 'uint32', type: 'uint32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_lib', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_configType', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'getConfig',
    outputs: [{ name: 'config', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'getReceiveLibrary',
    outputs: [
      { name: 'lib', internalType: 'address', type: 'address' },
      { name: 'isDefault', internalType: 'bool', type: 'bool' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getRegisteredLibraries',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getSendContext',
    outputs: [
      { name: 'dstEid', internalType: 'uint32', type: 'uint32' },
      { name: 'sender', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_sender', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'getSendLibrary',
    outputs: [{ name: 'lib', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'inboundNonce',
    outputs: [{ name: '', internalType: 'uint64', type: 'uint64' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
      { name: '_nonce', internalType: 'uint64', type: 'uint64' },
    ],
    name: 'inboundPayloadHash',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_origin',
        internalType: 'struct Origin',
        type: 'tuple',
        components: [
          { name: 'srcEid', internalType: 'uint32', type: 'uint32' },
          { name: 'sender', internalType: 'bytes32', type: 'bytes32' },
          { name: 'nonce', internalType: 'uint64', type: 'uint64' },
        ],
      },
      { name: '_receiver', internalType: 'address', type: 'address' },
    ],
    name: 'initializable',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_sender', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'isDefaultSendLibrary',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_lib', internalType: 'address', type: 'address' }],
    name: 'isRegisteredLibrary',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'isSendingMessage',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_eid', internalType: 'uint32', type: 'uint32' }],
    name: 'isSupportedEid',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_lib', internalType: 'address', type: 'address' },
    ],
    name: 'isValidReceiveLibrary',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'lazyInboundNonce',
    outputs: [{ name: '', internalType: 'uint64', type: 'uint64' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_from', internalType: 'address', type: 'address' },
      { name: '_to', internalType: 'address', type: 'address' },
      { name: '_guid', internalType: 'bytes32', type: 'bytes32' },
      { name: '_index', internalType: 'uint16', type: 'uint16' },
      { name: '_message', internalType: 'bytes', type: 'bytes' },
      { name: '_extraData', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'lzCompose',
    outputs: [],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_origin',
        internalType: 'struct Origin',
        type: 'tuple',
        components: [
          { name: 'srcEid', internalType: 'uint32', type: 'uint32' },
          { name: 'sender', internalType: 'bytes32', type: 'bytes32' },
          { name: 'nonce', internalType: 'uint64', type: 'uint64' },
        ],
      },
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_guid', internalType: 'bytes32', type: 'bytes32' },
      { name: '_message', internalType: 'bytes', type: 'bytes' },
      { name: '_extraData', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'lzReceive',
    outputs: [],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'lzToken',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'nativeToken',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_sender', internalType: 'address', type: 'address' },
      { name: '_dstEid', internalType: 'uint32', type: 'uint32' },
      { name: '_receiver', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'nextGuid',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
      { name: '_nonce', internalType: 'uint64', type: 'uint64' },
      { name: '_payloadHash', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'nilify',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_sender', internalType: 'address', type: 'address' },
      { name: '_dstEid', internalType: 'uint32', type: 'uint32' },
      { name: '_receiver', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'outboundNonce',
    outputs: [{ name: '', internalType: 'uint64', type: 'uint64' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_params',
        internalType: 'struct MessagingParams',
        type: 'tuple',
        components: [
          { name: 'dstEid', internalType: 'uint32', type: 'uint32' },
          { name: 'receiver', internalType: 'bytes32', type: 'bytes32' },
          { name: 'message', internalType: 'bytes', type: 'bytes' },
          { name: 'options', internalType: 'bytes', type: 'bytes' },
          { name: 'payInLzToken', internalType: 'bool', type: 'bool' },
        ],
      },
      { name: '_sender', internalType: 'address', type: 'address' },
    ],
    name: 'quote',
    outputs: [
      {
        name: '',
        internalType: 'struct MessagingFee',
        type: 'tuple',
        components: [
          { name: 'nativeFee', internalType: 'uint256', type: 'uint256' },
          { name: 'lzTokenFee', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'receiveLibraryTimeout',
    outputs: [
      { name: 'lib', internalType: 'address', type: 'address' },
      { name: 'expiry', internalType: 'uint256', type: 'uint256' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_lib', internalType: 'address', type: 'address' }],
    name: 'registerLibrary',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_params',
        internalType: 'struct MessagingParams',
        type: 'tuple',
        components: [
          { name: 'dstEid', internalType: 'uint32', type: 'uint32' },
          { name: 'receiver', internalType: 'bytes32', type: 'bytes32' },
          { name: 'message', internalType: 'bytes', type: 'bytes' },
          { name: 'options', internalType: 'bytes', type: 'bytes' },
          { name: 'payInLzToken', internalType: 'bool', type: 'bool' },
        ],
      },
      { name: '_refundAddress', internalType: 'address', type: 'address' },
    ],
    name: 'send',
    outputs: [
      {
        name: '',
        internalType: 'struct MessagingReceipt',
        type: 'tuple',
        components: [
          { name: 'guid', internalType: 'bytes32', type: 'bytes32' },
          { name: 'nonce', internalType: 'uint64', type: 'uint64' },
          {
            name: 'fee',
            internalType: 'struct MessagingFee',
            type: 'tuple',
            components: [
              { name: 'nativeFee', internalType: 'uint256', type: 'uint256' },
              { name: 'lzTokenFee', internalType: 'uint256', type: 'uint256' },
            ],
          },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_to', internalType: 'address', type: 'address' },
      { name: '_guid', internalType: 'bytes32', type: 'bytes32' },
      { name: '_index', internalType: 'uint16', type: 'uint16' },
      { name: '_message', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'sendCompose',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_lib', internalType: 'address', type: 'address' },
      {
        name: '_params',
        internalType: 'struct SetConfigParam[]',
        type: 'tuple[]',
        components: [
          { name: 'eid', internalType: 'uint32', type: 'uint32' },
          { name: 'configType', internalType: 'uint32', type: 'uint32' },
          { name: 'config', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'setConfig',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_newLib', internalType: 'address', type: 'address' },
      { name: '_timeout', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setDefaultReceiveLibrary',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_lib', internalType: 'address', type: 'address' },
      { name: '_expiry', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setDefaultReceiveLibraryTimeout',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_newLib', internalType: 'address', type: 'address' },
    ],
    name: 'setDefaultSendLibrary',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '_delegate', internalType: 'address', type: 'address' }],
    name: 'setDelegate',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '_lzToken', internalType: 'address', type: 'address' }],
    name: 'setLzToken',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_newLib', internalType: 'address', type: 'address' },
      { name: '_gracePeriod', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setReceiveLibrary',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_lib', internalType: 'address', type: 'address' },
      { name: '_gracePeriod', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setReceiveLibraryTimeout',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_newLib', internalType: 'address', type: 'address' },
    ],
    name: 'setSendLibrary',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
      { name: '_nonce', internalType: 'uint64', type: 'uint64' },
    ],
    name: 'skip',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_origin',
        internalType: 'struct Origin',
        type: 'tuple',
        components: [
          { name: 'srcEid', internalType: 'uint32', type: 'uint32' },
          { name: 'sender', internalType: 'bytes32', type: 'bytes32' },
          { name: 'nonce', internalType: 'uint64', type: 'uint64' },
        ],
      },
      { name: '_receiver', internalType: 'address', type: 'address' },
    ],
    name: 'verifiable',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_origin',
        internalType: 'struct Origin',
        type: 'tuple',
        components: [
          { name: 'srcEid', internalType: 'uint32', type: 'uint32' },
          { name: 'sender', internalType: 'bytes32', type: 'bytes32' },
          { name: 'nonce', internalType: 'uint64', type: 'uint64' },
        ],
      },
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_payloadHash', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'verify',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'from',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'to', internalType: 'address', type: 'address', indexed: false },
      {
        name: 'guid',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      { name: 'index', internalType: 'uint16', type: 'uint16', indexed: false },
    ],
    name: 'ComposeDelivered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'from',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'to', internalType: 'address', type: 'address', indexed: false },
      {
        name: 'guid',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      { name: 'index', internalType: 'uint16', type: 'uint16', indexed: false },
      { name: 'message', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'ComposeSent',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'newLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'DefaultReceiveLibrarySet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'oldLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'expiry',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'DefaultReceiveLibraryTimeoutSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'newLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'DefaultSendLibrarySet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'delegate',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'DelegateSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'srcEid',
        internalType: 'uint32',
        type: 'uint32',
        indexed: false,
      },
      {
        name: 'sender',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'nonce', internalType: 'uint64', type: 'uint64', indexed: false },
    ],
    name: 'InboundNonceSkipped',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'LibraryRegistered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'executor',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'guid',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      { name: 'index', internalType: 'uint16', type: 'uint16', indexed: false },
      { name: 'gas', internalType: 'uint256', type: 'uint256', indexed: false },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'message', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'extraData',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
      { name: 'reason', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'LzComposeAlert',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'executor',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'origin',
        internalType: 'struct Origin',
        type: 'tuple',
        components: [
          { name: 'srcEid', internalType: 'uint32', type: 'uint32' },
          { name: 'sender', internalType: 'bytes32', type: 'bytes32' },
          { name: 'nonce', internalType: 'uint64', type: 'uint64' },
        ],
        indexed: false,
      },
      {
        name: 'guid',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      { name: 'gas', internalType: 'uint256', type: 'uint256', indexed: false },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'message', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'extraData',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
      { name: 'reason', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'LzReceiveAlert',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'token',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'LzTokenSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'srcEid',
        internalType: 'uint32',
        type: 'uint32',
        indexed: false,
      },
      {
        name: 'sender',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'nonce', internalType: 'uint64', type: 'uint64', indexed: false },
      {
        name: 'payloadHash',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'PacketBurnt',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'origin',
        internalType: 'struct Origin',
        type: 'tuple',
        components: [
          { name: 'srcEid', internalType: 'uint32', type: 'uint32' },
          { name: 'sender', internalType: 'bytes32', type: 'bytes32' },
          { name: 'nonce', internalType: 'uint64', type: 'uint64' },
        ],
        indexed: false,
      },
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'PacketDelivered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'srcEid',
        internalType: 'uint32',
        type: 'uint32',
        indexed: false,
      },
      {
        name: 'sender',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'nonce', internalType: 'uint64', type: 'uint64', indexed: false },
      {
        name: 'payloadHash',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'PacketNilified',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'encodedPayload',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
      { name: 'options', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'sendLibrary',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'PacketSent',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'origin',
        internalType: 'struct Origin',
        type: 'tuple',
        components: [
          { name: 'srcEid', internalType: 'uint32', type: 'uint32' },
          { name: 'sender', internalType: 'bytes32', type: 'bytes32' },
          { name: 'nonce', internalType: 'uint64', type: 'uint64' },
        ],
        indexed: false,
      },
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'payloadHash',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'PacketVerified',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'newLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ReceiveLibrarySet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'oldLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'timeout',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ReceiveLibraryTimeoutSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'newLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'SendLibrarySet',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ILegacyMeshOFT
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iLegacyMeshOftAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'BPS_DENOMINATOR',
    outputs: [{ name: '', internalType: 'uint16', type: 'uint16' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'approvalRequired',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'feeBps',
    outputs: [{ name: '', internalType: 'uint16', type: 'uint16' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'oftVersion',
    outputs: [
      { name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' },
      { name: 'version', internalType: 'uint64', type: 'uint64' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_sendParam',
        internalType: 'struct SendParam',
        type: 'tuple',
        components: [
          { name: 'dstEid', internalType: 'uint32', type: 'uint32' },
          { name: 'to', internalType: 'bytes32', type: 'bytes32' },
          { name: 'amountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'minAmountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'extraOptions', internalType: 'bytes', type: 'bytes' },
          { name: 'composeMsg', internalType: 'bytes', type: 'bytes' },
          { name: 'oftCmd', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'quoteOFT',
    outputs: [
      {
        name: '',
        internalType: 'struct OFTLimit',
        type: 'tuple',
        components: [
          { name: 'minAmountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'maxAmountLD', internalType: 'uint256', type: 'uint256' },
        ],
      },
      {
        name: 'oftFeeDetails',
        internalType: 'struct OFTFeeDetail[]',
        type: 'tuple[]',
        components: [
          { name: 'feeAmountLD', internalType: 'int256', type: 'int256' },
          { name: 'description', internalType: 'string', type: 'string' },
        ],
      },
      {
        name: '',
        internalType: 'struct OFTReceipt',
        type: 'tuple',
        components: [
          { name: 'amountSentLD', internalType: 'uint256', type: 'uint256' },
          {
            name: 'amountReceivedLD',
            internalType: 'uint256',
            type: 'uint256',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_sendParam',
        internalType: 'struct SendParam',
        type: 'tuple',
        components: [
          { name: 'dstEid', internalType: 'uint32', type: 'uint32' },
          { name: 'to', internalType: 'bytes32', type: 'bytes32' },
          { name: 'amountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'minAmountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'extraOptions', internalType: 'bytes', type: 'bytes' },
          { name: 'composeMsg', internalType: 'bytes', type: 'bytes' },
          { name: 'oftCmd', internalType: 'bytes', type: 'bytes' },
        ],
      },
      { name: '_payInLzToken', internalType: 'bool', type: 'bool' },
    ],
    name: 'quoteSend',
    outputs: [
      {
        name: '',
        internalType: 'struct MessagingFee',
        type: 'tuple',
        components: [
          { name: 'nativeFee', internalType: 'uint256', type: 'uint256' },
          { name: 'lzTokenFee', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_sendParam',
        internalType: 'struct SendParam',
        type: 'tuple',
        components: [
          { name: 'dstEid', internalType: 'uint32', type: 'uint32' },
          { name: 'to', internalType: 'bytes32', type: 'bytes32' },
          { name: 'amountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'minAmountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'extraOptions', internalType: 'bytes', type: 'bytes' },
          { name: 'composeMsg', internalType: 'bytes', type: 'bytes' },
          { name: 'oftCmd', internalType: 'bytes', type: 'bytes' },
        ],
      },
      {
        name: '_fee',
        internalType: 'struct MessagingFee',
        type: 'tuple',
        components: [
          { name: 'nativeFee', internalType: 'uint256', type: 'uint256' },
          { name: 'lzTokenFee', internalType: 'uint256', type: 'uint256' },
        ],
      },
      { name: '_refundAddress', internalType: 'address', type: 'address' },
    ],
    name: 'send',
    outputs: [
      {
        name: '',
        internalType: 'struct MessagingReceipt',
        type: 'tuple',
        components: [
          { name: 'guid', internalType: 'bytes32', type: 'bytes32' },
          { name: 'nonce', internalType: 'uint64', type: 'uint64' },
          {
            name: 'fee',
            internalType: 'struct MessagingFee',
            type: 'tuple',
            components: [
              { name: 'nativeFee', internalType: 'uint256', type: 'uint256' },
              { name: 'lzTokenFee', internalType: 'uint256', type: 'uint256' },
            ],
          },
        ],
      },
      {
        name: '',
        internalType: 'struct OFTReceipt',
        type: 'tuple',
        components: [
          { name: 'amountSentLD', internalType: 'uint256', type: 'uint256' },
          {
            name: 'amountReceivedLD',
            internalType: 'uint256',
            type: 'uint256',
          },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'sharedDecimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'token',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'guid', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'srcEid',
        internalType: 'uint32',
        type: 'uint32',
        indexed: false,
      },
      {
        name: 'toAddress',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amountReceivedLD',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'OFTReceived',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'guid', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'dstEid',
        internalType: 'uint32',
        type: 'uint32',
        indexed: false,
      },
      {
        name: 'fromAddress',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amountSentLD',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'amountReceivedLD',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'OFTSent',
  },
  {
    type: 'error',
    inputs: [{ name: 'amountSD', internalType: 'uint256', type: 'uint256' }],
    name: 'AmountSDOverflowed',
  },
  { type: 'error', inputs: [], name: 'InvalidLocalDecimals' },
  {
    type: 'error',
    inputs: [
      { name: 'amountLD', internalType: 'uint256', type: 'uint256' },
      { name: 'minAmountLD', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'SlippageExceeded',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IMessageLibManager
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iMessageLibManagerAbi = [
  {
    type: 'function',
    inputs: [{ name: '_eid', internalType: 'uint32', type: 'uint32' }],
    name: 'defaultReceiveLibrary',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_eid', internalType: 'uint32', type: 'uint32' }],
    name: 'defaultReceiveLibraryTimeout',
    outputs: [
      { name: 'lib', internalType: 'address', type: 'address' },
      { name: 'expiry', internalType: 'uint256', type: 'uint256' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_eid', internalType: 'uint32', type: 'uint32' }],
    name: 'defaultSendLibrary',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_lib', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_configType', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'getConfig',
    outputs: [{ name: 'config', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'getReceiveLibrary',
    outputs: [
      { name: 'lib', internalType: 'address', type: 'address' },
      { name: 'isDefault', internalType: 'bool', type: 'bool' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getRegisteredLibraries',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_sender', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'getSendLibrary',
    outputs: [{ name: 'lib', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_sender', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'isDefaultSendLibrary',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_lib', internalType: 'address', type: 'address' }],
    name: 'isRegisteredLibrary',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_eid', internalType: 'uint32', type: 'uint32' }],
    name: 'isSupportedEid',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_lib', internalType: 'address', type: 'address' },
    ],
    name: 'isValidReceiveLibrary',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'receiveLibraryTimeout',
    outputs: [
      { name: 'lib', internalType: 'address', type: 'address' },
      { name: 'expiry', internalType: 'uint256', type: 'uint256' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_lib', internalType: 'address', type: 'address' }],
    name: 'registerLibrary',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_lib', internalType: 'address', type: 'address' },
      {
        name: '_params',
        internalType: 'struct SetConfigParam[]',
        type: 'tuple[]',
        components: [
          { name: 'eid', internalType: 'uint32', type: 'uint32' },
          { name: 'configType', internalType: 'uint32', type: 'uint32' },
          { name: 'config', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'setConfig',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_newLib', internalType: 'address', type: 'address' },
      { name: '_timeout', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setDefaultReceiveLibrary',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_lib', internalType: 'address', type: 'address' },
      { name: '_expiry', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setDefaultReceiveLibraryTimeout',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_newLib', internalType: 'address', type: 'address' },
    ],
    name: 'setDefaultSendLibrary',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_newLib', internalType: 'address', type: 'address' },
      { name: '_gracePeriod', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setReceiveLibrary',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_lib', internalType: 'address', type: 'address' },
      { name: '_gracePeriod', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setReceiveLibraryTimeout',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_newLib', internalType: 'address', type: 'address' },
    ],
    name: 'setSendLibrary',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'newLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'DefaultReceiveLibrarySet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'oldLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'expiry',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'DefaultReceiveLibraryTimeoutSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'newLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'DefaultSendLibrarySet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'LibraryRegistered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'newLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ReceiveLibrarySet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'oldLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'timeout',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ReceiveLibraryTimeoutSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'newLib',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'SendLibrarySet',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IMessagingChannel
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iMessagingChannelAbi = [
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
      { name: '_nonce', internalType: 'uint64', type: 'uint64' },
      { name: '_payloadHash', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'burn',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'eid',
    outputs: [{ name: '', internalType: 'uint32', type: 'uint32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'inboundNonce',
    outputs: [{ name: '', internalType: 'uint64', type: 'uint64' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
      { name: '_nonce', internalType: 'uint64', type: 'uint64' },
    ],
    name: 'inboundPayloadHash',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_receiver', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'lazyInboundNonce',
    outputs: [{ name: '', internalType: 'uint64', type: 'uint64' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_sender', internalType: 'address', type: 'address' },
      { name: '_dstEid', internalType: 'uint32', type: 'uint32' },
      { name: '_receiver', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'nextGuid',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
      { name: '_nonce', internalType: 'uint64', type: 'uint64' },
      { name: '_payloadHash', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'nilify',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_sender', internalType: 'address', type: 'address' },
      { name: '_dstEid', internalType: 'uint32', type: 'uint32' },
      { name: '_receiver', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'outboundNonce',
    outputs: [{ name: '', internalType: 'uint64', type: 'uint64' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_oapp', internalType: 'address', type: 'address' },
      { name: '_srcEid', internalType: 'uint32', type: 'uint32' },
      { name: '_sender', internalType: 'bytes32', type: 'bytes32' },
      { name: '_nonce', internalType: 'uint64', type: 'uint64' },
    ],
    name: 'skip',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'srcEid',
        internalType: 'uint32',
        type: 'uint32',
        indexed: false,
      },
      {
        name: 'sender',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'nonce', internalType: 'uint64', type: 'uint64', indexed: false },
    ],
    name: 'InboundNonceSkipped',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'srcEid',
        internalType: 'uint32',
        type: 'uint32',
        indexed: false,
      },
      {
        name: 'sender',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'nonce', internalType: 'uint64', type: 'uint64', indexed: false },
      {
        name: 'payloadHash',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'PacketBurnt',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'srcEid',
        internalType: 'uint32',
        type: 'uint32',
        indexed: false,
      },
      {
        name: 'sender',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'nonce', internalType: 'uint64', type: 'uint64', indexed: false },
      {
        name: 'payloadHash',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'PacketNilified',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IMessagingComposer
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iMessagingComposerAbi = [
  {
    type: 'function',
    inputs: [
      { name: '_from', internalType: 'address', type: 'address' },
      { name: '_to', internalType: 'address', type: 'address' },
      { name: '_guid', internalType: 'bytes32', type: 'bytes32' },
      { name: '_index', internalType: 'uint16', type: 'uint16' },
    ],
    name: 'composeQueue',
    outputs: [
      { name: 'messageHash', internalType: 'bytes32', type: 'bytes32' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '_from', internalType: 'address', type: 'address' },
      { name: '_to', internalType: 'address', type: 'address' },
      { name: '_guid', internalType: 'bytes32', type: 'bytes32' },
      { name: '_index', internalType: 'uint16', type: 'uint16' },
      { name: '_message', internalType: 'bytes', type: 'bytes' },
      { name: '_extraData', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'lzCompose',
    outputs: [],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_to', internalType: 'address', type: 'address' },
      { name: '_guid', internalType: 'bytes32', type: 'bytes32' },
      { name: '_index', internalType: 'uint16', type: 'uint16' },
      { name: '_message', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'sendCompose',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'from',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'to', internalType: 'address', type: 'address', indexed: false },
      {
        name: 'guid',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      { name: 'index', internalType: 'uint16', type: 'uint16', indexed: false },
    ],
    name: 'ComposeDelivered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'from',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      { name: 'to', internalType: 'address', type: 'address', indexed: false },
      {
        name: 'guid',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      { name: 'index', internalType: 'uint16', type: 'uint16', indexed: false },
      { name: 'message', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'ComposeSent',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'executor',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'guid',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      { name: 'index', internalType: 'uint16', type: 'uint16', indexed: false },
      { name: 'gas', internalType: 'uint256', type: 'uint256', indexed: false },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'message', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'extraData',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
      { name: 'reason', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'LzComposeAlert',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IMessagingContext
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iMessagingContextAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'getSendContext',
    outputs: [
      { name: 'dstEid', internalType: 'uint32', type: 'uint32' },
      { name: 'sender', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'isSendingMessage',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IOAppCore
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ioAppCoreAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'endpoint',
    outputs: [
      {
        name: 'iEndpoint',
        internalType: 'contract ILayerZeroEndpointV2',
        type: 'address',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'oAppVersion',
    outputs: [
      { name: 'senderVersion', internalType: 'uint64', type: 'uint64' },
      { name: 'receiverVersion', internalType: 'uint64', type: 'uint64' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_eid', internalType: 'uint32', type: 'uint32' }],
    name: 'peers',
    outputs: [{ name: 'peer', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_delegate', internalType: 'address', type: 'address' }],
    name: 'setDelegate',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_peer', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'setPeer',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'peer',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'PeerSet',
  },
  { type: 'error', inputs: [], name: 'InvalidDelegate' },
  { type: 'error', inputs: [], name: 'InvalidEndpointCall' },
  {
    type: 'error',
    inputs: [{ name: 'eid', internalType: 'uint32', type: 'uint32' }],
    name: 'NoPeer',
  },
  {
    type: 'error',
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32' },
      { name: 'sender', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'OnlyPeer',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IOFT
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ioftAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'approvalRequired',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'oftVersion',
    outputs: [
      { name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' },
      { name: 'version', internalType: 'uint64', type: 'uint64' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_sendParam',
        internalType: 'struct SendParam',
        type: 'tuple',
        components: [
          { name: 'dstEid', internalType: 'uint32', type: 'uint32' },
          { name: 'to', internalType: 'bytes32', type: 'bytes32' },
          { name: 'amountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'minAmountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'extraOptions', internalType: 'bytes', type: 'bytes' },
          { name: 'composeMsg', internalType: 'bytes', type: 'bytes' },
          { name: 'oftCmd', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'quoteOFT',
    outputs: [
      {
        name: '',
        internalType: 'struct OFTLimit',
        type: 'tuple',
        components: [
          { name: 'minAmountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'maxAmountLD', internalType: 'uint256', type: 'uint256' },
        ],
      },
      {
        name: 'oftFeeDetails',
        internalType: 'struct OFTFeeDetail[]',
        type: 'tuple[]',
        components: [
          { name: 'feeAmountLD', internalType: 'int256', type: 'int256' },
          { name: 'description', internalType: 'string', type: 'string' },
        ],
      },
      {
        name: '',
        internalType: 'struct OFTReceipt',
        type: 'tuple',
        components: [
          { name: 'amountSentLD', internalType: 'uint256', type: 'uint256' },
          {
            name: 'amountReceivedLD',
            internalType: 'uint256',
            type: 'uint256',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_sendParam',
        internalType: 'struct SendParam',
        type: 'tuple',
        components: [
          { name: 'dstEid', internalType: 'uint32', type: 'uint32' },
          { name: 'to', internalType: 'bytes32', type: 'bytes32' },
          { name: 'amountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'minAmountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'extraOptions', internalType: 'bytes', type: 'bytes' },
          { name: 'composeMsg', internalType: 'bytes', type: 'bytes' },
          { name: 'oftCmd', internalType: 'bytes', type: 'bytes' },
        ],
      },
      { name: '_payInLzToken', internalType: 'bool', type: 'bool' },
    ],
    name: 'quoteSend',
    outputs: [
      {
        name: '',
        internalType: 'struct MessagingFee',
        type: 'tuple',
        components: [
          { name: 'nativeFee', internalType: 'uint256', type: 'uint256' },
          { name: 'lzTokenFee', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_sendParam',
        internalType: 'struct SendParam',
        type: 'tuple',
        components: [
          { name: 'dstEid', internalType: 'uint32', type: 'uint32' },
          { name: 'to', internalType: 'bytes32', type: 'bytes32' },
          { name: 'amountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'minAmountLD', internalType: 'uint256', type: 'uint256' },
          { name: 'extraOptions', internalType: 'bytes', type: 'bytes' },
          { name: 'composeMsg', internalType: 'bytes', type: 'bytes' },
          { name: 'oftCmd', internalType: 'bytes', type: 'bytes' },
        ],
      },
      {
        name: '_fee',
        internalType: 'struct MessagingFee',
        type: 'tuple',
        components: [
          { name: 'nativeFee', internalType: 'uint256', type: 'uint256' },
          { name: 'lzTokenFee', internalType: 'uint256', type: 'uint256' },
        ],
      },
      { name: '_refundAddress', internalType: 'address', type: 'address' },
    ],
    name: 'send',
    outputs: [
      {
        name: '',
        internalType: 'struct MessagingReceipt',
        type: 'tuple',
        components: [
          { name: 'guid', internalType: 'bytes32', type: 'bytes32' },
          { name: 'nonce', internalType: 'uint64', type: 'uint64' },
          {
            name: 'fee',
            internalType: 'struct MessagingFee',
            type: 'tuple',
            components: [
              { name: 'nativeFee', internalType: 'uint256', type: 'uint256' },
              { name: 'lzTokenFee', internalType: 'uint256', type: 'uint256' },
            ],
          },
        ],
      },
      {
        name: '',
        internalType: 'struct OFTReceipt',
        type: 'tuple',
        components: [
          { name: 'amountSentLD', internalType: 'uint256', type: 'uint256' },
          {
            name: 'amountReceivedLD',
            internalType: 'uint256',
            type: 'uint256',
          },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'sharedDecimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'token',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'guid', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'srcEid',
        internalType: 'uint32',
        type: 'uint32',
        indexed: false,
      },
      {
        name: 'toAddress',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amountReceivedLD',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'OFTReceived',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'guid', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'dstEid',
        internalType: 'uint32',
        type: 'uint32',
        indexed: false,
      },
      {
        name: 'fromAddress',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amountSentLD',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'amountReceivedLD',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'OFTSent',
  },
  {
    type: 'error',
    inputs: [{ name: 'amountSD', internalType: 'uint256', type: 'uint256' }],
    name: 'AmountSDOverflowed',
  },
  { type: 'error', inputs: [], name: 'InvalidLocalDecimals' },
  {
    type: 'error',
    inputs: [
      { name: 'amountLD', internalType: 'uint256', type: 'uint256' },
      { name: 'minAmountLD', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'SlippageExceeded',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// LegacyMeshBridger
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const legacyMeshBridgerAbi = [
  {
    type: 'function',
    inputs: [
      { name: 'token', internalType: 'address', type: 'address' },
      { name: 'inAmount', internalType: 'uint256', type: 'uint256' },
      { name: 'payload', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'bridge',
    outputs: [{ name: 'outAmount', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'nonpayable',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// OAppCore
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const oAppCoreAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'endpoint',
    outputs: [
      {
        name: '',
        internalType: 'contract ILayerZeroEndpointV2',
        type: 'address',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'oAppVersion',
    outputs: [
      { name: 'senderVersion', internalType: 'uint64', type: 'uint64' },
      { name: 'receiverVersion', internalType: 'uint64', type: 'uint64' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'eid', internalType: 'uint32', type: 'uint32' }],
    name: 'peers',
    outputs: [{ name: 'peer', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'renounceOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '_delegate', internalType: 'address', type: 'address' }],
    name: 'setDelegate',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_peer', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'setPeer',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'peer',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'PeerSet',
  },
  { type: 'error', inputs: [], name: 'InvalidDelegate' },
  { type: 'error', inputs: [], name: 'InvalidEndpointCall' },
  {
    type: 'error',
    inputs: [{ name: 'eid', internalType: 'uint32', type: 'uint32' }],
    name: 'NoPeer',
  },
  {
    type: 'error',
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32' },
      { name: 'sender', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'OnlyPeer',
  },
  {
    type: 'error',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'OwnableInvalidOwner',
  },
  {
    type: 'error',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'OwnableUnauthorizedAccount',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// OAppSender
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const oAppSenderAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'endpoint',
    outputs: [
      {
        name: '',
        internalType: 'contract ILayerZeroEndpointV2',
        type: 'address',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'oAppVersion',
    outputs: [
      { name: 'senderVersion', internalType: 'uint64', type: 'uint64' },
      { name: 'receiverVersion', internalType: 'uint64', type: 'uint64' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'eid', internalType: 'uint32', type: 'uint32' }],
    name: 'peers',
    outputs: [{ name: 'peer', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'renounceOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '_delegate', internalType: 'address', type: 'address' }],
    name: 'setDelegate',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_eid', internalType: 'uint32', type: 'uint32' },
      { name: '_peer', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'setPeer',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32', indexed: false },
      {
        name: 'peer',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'PeerSet',
  },
  { type: 'error', inputs: [], name: 'InvalidDelegate' },
  { type: 'error', inputs: [], name: 'InvalidEndpointCall' },
  { type: 'error', inputs: [], name: 'LzTokenUnavailable' },
  {
    type: 'error',
    inputs: [{ name: 'eid', internalType: 'uint32', type: 'uint32' }],
    name: 'NoPeer',
  },
  {
    type: 'error',
    inputs: [{ name: 'msgValue', internalType: 'uint256', type: 'uint256' }],
    name: 'NotEnoughNative',
  },
  {
    type: 'error',
    inputs: [
      { name: 'eid', internalType: 'uint32', type: 'uint32' },
      { name: 'sender', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'OnlyPeer',
  },
  {
    type: 'error',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'OwnableInvalidOwner',
  },
  {
    type: 'error',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'OwnableUnauthorizedAccount',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Ownable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ownableAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'renounceOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'error',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'OwnableInvalidOwner',
  },
  {
    type: 'error',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'OwnableUnauthorizedAccount',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// SafeERC20
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const safeErc20Abi = [
  {
    type: 'error',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'currentAllowance', internalType: 'uint256', type: 'uint256' },
      { name: 'requestedDecrease', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'SafeERC20FailedDecreaseAllowance',
  },
  {
    type: 'error',
    inputs: [{ name: 'token', internalType: 'address', type: 'address' }],
    name: 'SafeERC20FailedOperation',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// SafeTransferLib
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const safeTransferLibAbi = [
  { type: 'error', inputs: [], name: 'ApproveFailed' },
  { type: 'error', inputs: [], name: 'ETHTransferFailed' },
  { type: 'error', inputs: [], name: 'Permit2AmountOverflow' },
  { type: 'error', inputs: [], name: 'Permit2ApproveFailed' },
  { type: 'error', inputs: [], name: 'Permit2Failed' },
  { type: 'error', inputs: [], name: 'Permit2LockdownFailed' },
  { type: 'error', inputs: [], name: 'TotalSupplyQueryFailed' },
  { type: 'error', inputs: [], name: 'TransferFailed' },
  { type: 'error', inputs: [], name: 'TransferFromFailed' },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TronLightClient
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const tronLightClientAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: 'blockRangeProver',
        internalType: 'contract IBlockRangeProver',
        type: 'address',
      },
      { name: 'initialBlockHash', internalType: 'bytes32', type: 'bytes32' },
      { name: 'initialTxTrieRoot', internalType: 'bytes32', type: 'bytes32' },
      { name: 'initialTimestamp', internalType: 'uint32', type: 'uint32' },
      { name: '_srs', internalType: 'bytes20[27]', type: 'bytes20[27]' },
      {
        name: '_witnessDelegatees',
        internalType: 'bytes20[27]',
        type: 'bytes20[27]',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'BLOCK_RANGE_PROVER',
    outputs: [
      { name: '', internalType: 'contract IBlockRangeProver', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'blockNumber', internalType: 'uint256', type: 'uint256' }],
    name: 'getBlockId',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'blockNumber', internalType: 'uint256', type: 'uint256' }],
    name: 'getBlockTimestamp',
    outputs: [{ name: '', internalType: 'uint32', type: 'uint32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'blockNumber', internalType: 'uint256', type: 'uint256' }],
    name: 'getTxTrieRoot',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'latestProvenBlock',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'startingBlock', internalType: 'bytes32', type: 'bytes32' },
      { name: 'endingBlock', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'endingBlockTxTrieRoot',
        internalType: 'bytes32',
        type: 'bytes32',
      },
      { name: 'endingBlockTimestamp', internalType: 'uint32', type: 'uint32' },
      { name: 'zkProof', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'proveBlockRange',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'startingBlock', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'compressedTronBlockMetadata',
        internalType: 'bytes',
        type: 'bytes',
      },
      { name: 'compressedSignatures', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'proveBlocks',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    name: 'srs',
    outputs: [{ name: '', internalType: 'bytes20', type: 'bytes20' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    name: 'witnessDelegatees',
    outputs: [{ name: '', internalType: 'bytes20', type: 'bytes20' }],
    stateMutability: 'view',
  },
  { type: 'error', inputs: [], name: 'BlockNotRelayed' },
  { type: 'error', inputs: [], name: 'BlockTooOld' },
  { type: 'error', inputs: [], name: 'ECDSAInvalidSignature' },
  {
    type: 'error',
    inputs: [{ name: 'length', internalType: 'uint256', type: 'uint256' }],
    name: 'ECDSAInvalidSignatureLength',
  },
  {
    type: 'error',
    inputs: [{ name: 's', internalType: 'bytes32', type: 'bytes32' }],
    name: 'ECDSAInvalidSignatureS',
  },
  { type: 'error', inputs: [], name: 'InvalidChain' },
  { type: 'error', inputs: [], name: 'InvalidCompressedSignaturesLength' },
  {
    type: 'error',
    inputs: [],
    name: 'InvalidCompressedTronBlockMetadataLength',
  },
  {
    type: 'error',
    inputs: [
      { name: 'yours', internalType: 'bytes32', type: 'bytes32' },
      { name: 'real', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'InvalidParentBlockId',
  },
  { type: 'error', inputs: [], name: 'InvalidWitnessSigner' },
  { type: 'error', inputs: [], name: 'NotEnoughBlocksOrSignatures' },
  { type: 'error', inputs: [], name: 'UnanchoredBlockRange' },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TronLightClientHarness
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const tronLightClientHarnessAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: 'p',
        internalType: 'contract IBlockRangeProver',
        type: 'address',
      },
      { name: 'initial', internalType: 'bytes32', type: 'bytes32' },
      { name: 'initialTxTrieRoot', internalType: 'bytes32', type: 'bytes32' },
      { name: 'initialTimestamp', internalType: 'uint32', type: 'uint32' },
      { name: 'srs_', internalType: 'bytes20[27]', type: 'bytes20[27]' },
      {
        name: 'witnessDelegatees_',
        internalType: 'bytes20[27]',
        type: 'bytes20[27]',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'BLOCK_RANGE_PROVER',
    outputs: [
      { name: '', internalType: 'contract IBlockRangeProver', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'data', internalType: 'bytes', type: 'bytes' },
      { name: 'idx', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'decodeAt',
    outputs: [
      {
        name: '',
        internalType: 'struct TronLightClient.TronBlockMetadata',
        type: 'tuple',
        components: [
          { name: 'parentHash', internalType: 'bytes32', type: 'bytes32' },
          { name: 'txTrieRoot', internalType: 'bytes32', type: 'bytes32' },
          { name: 'timestamp', internalType: 'uint32', type: 'uint32' },
          { name: 'witnessAddressIndex', internalType: 'uint8', type: 'uint8' },
        ],
      },
    ],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'b',
        internalType: 'struct TronLightClient.TronBlockMetadata',
        type: 'tuple',
        components: [
          { name: 'parentHash', internalType: 'bytes32', type: 'bytes32' },
          { name: 'txTrieRoot', internalType: 'bytes32', type: 'bytes32' },
          { name: 'timestamp', internalType: 'uint32', type: 'uint32' },
          { name: 'witnessAddressIndex', internalType: 'uint8', type: 'uint8' },
        ],
      },
      { name: 'n', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'encodeBlockHeaderPublic',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'blockNumber', internalType: 'uint256', type: 'uint256' }],
    name: 'getBlockId',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'blockNumber', internalType: 'uint256', type: 'uint256' }],
    name: 'getBlockTimestamp',
    outputs: [{ name: '', internalType: 'uint32', type: 'uint32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'blockNumber', internalType: 'uint256', type: 'uint256' }],
    name: 'getTxTrieRoot',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'b',
        internalType: 'struct TronLightClient.TronBlockMetadata',
        type: 'tuple',
        components: [
          { name: 'parentHash', internalType: 'bytes32', type: 'bytes32' },
          { name: 'txTrieRoot', internalType: 'bytes32', type: 'bytes32' },
          { name: 'timestamp', internalType: 'uint32', type: 'uint32' },
          { name: 'witnessAddressIndex', internalType: 'uint8', type: 'uint8' },
        ],
      },
      { name: 'n', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'hashBlockPublic',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'latestProvenBlock',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'startingBlock', internalType: 'bytes32', type: 'bytes32' },
      { name: 'endingBlock', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'endingBlockTxTrieRoot',
        internalType: 'bytes32',
        type: 'bytes32',
      },
      { name: 'endingBlockTimestamp', internalType: 'uint32', type: 'uint32' },
      { name: 'zkProof', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'proveBlockRange',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'startingBlock', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'compressedTronBlockMetadata',
        internalType: 'bytes',
        type: 'bytes',
      },
      { name: 'compressedSignatures', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'proveBlocks',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    name: 'srs',
    outputs: [{ name: '', internalType: 'bytes20', type: 'bytes20' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    name: 'witnessDelegatees',
    outputs: [{ name: '', internalType: 'bytes20', type: 'bytes20' }],
    stateMutability: 'view',
  },
  { type: 'error', inputs: [], name: 'BlockNotRelayed' },
  { type: 'error', inputs: [], name: 'BlockTooOld' },
  { type: 'error', inputs: [], name: 'ECDSAInvalidSignature' },
  {
    type: 'error',
    inputs: [{ name: 'length', internalType: 'uint256', type: 'uint256' }],
    name: 'ECDSAInvalidSignatureLength',
  },
  {
    type: 'error',
    inputs: [{ name: 's', internalType: 'bytes32', type: 'bytes32' }],
    name: 'ECDSAInvalidSignatureS',
  },
  { type: 'error', inputs: [], name: 'InvalidChain' },
  { type: 'error', inputs: [], name: 'InvalidCompressedSignaturesLength' },
  {
    type: 'error',
    inputs: [],
    name: 'InvalidCompressedTronBlockMetadataLength',
  },
  {
    type: 'error',
    inputs: [
      { name: 'yours', internalType: 'bytes32', type: 'bytes32' },
      { name: 'real', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'InvalidParentBlockId',
  },
  { type: 'error', inputs: [], name: 'InvalidWitnessSigner' },
  { type: 'error', inputs: [], name: 'NotEnoughBlocksOrSignatures' },
  { type: 'error', inputs: [], name: 'UnanchoredBlockRange' },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TronSha256MerkleVerifierHarness
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const tronSha256MerkleVerifierHarnessAbi = [
  {
    type: 'function',
    inputs: [
      { name: 'root', internalType: 'bytes32', type: 'bytes32' },
      { name: 'leaf', internalType: 'bytes32', type: 'bytes32' },
      { name: 'proof', internalType: 'bytes32[]', type: 'bytes32[]' },
      { name: 'index', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'verify',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'pure',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TronSha256MerkleVerifierTest
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const tronSha256MerkleVerifierTestAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'IS_TEST',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'excludeArtifacts',
    outputs: [
      {
        name: 'excludedArtifacts_',
        internalType: 'string[]',
        type: 'string[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'excludeContracts',
    outputs: [
      {
        name: 'excludedContracts_',
        internalType: 'address[]',
        type: 'address[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'excludeSelectors',
    outputs: [
      {
        name: 'excludedSelectors_',
        internalType: 'struct StdInvariant.FuzzSelector[]',
        type: 'tuple[]',
        components: [
          { name: 'addr', internalType: 'address', type: 'address' },
          { name: 'selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'excludeSenders',
    outputs: [
      {
        name: 'excludedSenders_',
        internalType: 'address[]',
        type: 'address[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'failed',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'setUp',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'targetArtifactSelectors',
    outputs: [
      {
        name: 'targetedArtifactSelectors_',
        internalType: 'struct StdInvariant.FuzzArtifactSelector[]',
        type: 'tuple[]',
        components: [
          { name: 'artifact', internalType: 'string', type: 'string' },
          { name: 'selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'targetArtifacts',
    outputs: [
      {
        name: 'targetedArtifacts_',
        internalType: 'string[]',
        type: 'string[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'targetContracts',
    outputs: [
      {
        name: 'targetedContracts_',
        internalType: 'address[]',
        type: 'address[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'targetInterfaces',
    outputs: [
      {
        name: 'targetedInterfaces_',
        internalType: 'struct StdInvariant.FuzzInterface[]',
        type: 'tuple[]',
        components: [
          { name: 'addr', internalType: 'address', type: 'address' },
          { name: 'artifacts', internalType: 'string[]', type: 'string[]' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'targetSelectors',
    outputs: [
      {
        name: 'targetedSelectors_',
        internalType: 'struct StdInvariant.FuzzSelector[]',
        type: 'tuple[]',
        components: [
          { name: 'addr', internalType: 'address', type: 'address' },
          { name: 'selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'targetSenders',
    outputs: [
      {
        name: 'targetedSenders_',
        internalType: 'address[]',
        type: 'address[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'test_verify_AcceptsValidFixtureProof',
    outputs: [],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'test_verify_RejectsWrongLeaf',
    outputs: [],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: '', internalType: 'string', type: 'string', indexed: false },
    ],
    name: 'log',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: '', internalType: 'address', type: 'address', indexed: false },
    ],
    name: 'log_address',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'val',
        internalType: 'uint256[]',
        type: 'uint256[]',
        indexed: false,
      },
    ],
    name: 'log_array',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'val',
        internalType: 'int256[]',
        type: 'int256[]',
        indexed: false,
      },
    ],
    name: 'log_array',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'val',
        internalType: 'address[]',
        type: 'address[]',
        indexed: false,
      },
    ],
    name: 'log_array',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: '', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'log_bytes',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: '', internalType: 'bytes32', type: 'bytes32', indexed: false },
    ],
    name: 'log_bytes32',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: '', internalType: 'int256', type: 'int256', indexed: false },
    ],
    name: 'log_int',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'string', type: 'string', indexed: false },
      { name: 'val', internalType: 'address', type: 'address', indexed: false },
    ],
    name: 'log_named_address',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'string', type: 'string', indexed: false },
      {
        name: 'val',
        internalType: 'uint256[]',
        type: 'uint256[]',
        indexed: false,
      },
    ],
    name: 'log_named_array',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'string', type: 'string', indexed: false },
      {
        name: 'val',
        internalType: 'int256[]',
        type: 'int256[]',
        indexed: false,
      },
    ],
    name: 'log_named_array',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'string', type: 'string', indexed: false },
      {
        name: 'val',
        internalType: 'address[]',
        type: 'address[]',
        indexed: false,
      },
    ],
    name: 'log_named_array',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'string', type: 'string', indexed: false },
      { name: 'val', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'log_named_bytes',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'string', type: 'string', indexed: false },
      { name: 'val', internalType: 'bytes32', type: 'bytes32', indexed: false },
    ],
    name: 'log_named_bytes32',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'string', type: 'string', indexed: false },
      { name: 'val', internalType: 'int256', type: 'int256', indexed: false },
      {
        name: 'decimals',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'log_named_decimal_int',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'string', type: 'string', indexed: false },
      { name: 'val', internalType: 'uint256', type: 'uint256', indexed: false },
      {
        name: 'decimals',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'log_named_decimal_uint',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'string', type: 'string', indexed: false },
      { name: 'val', internalType: 'int256', type: 'int256', indexed: false },
    ],
    name: 'log_named_int',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'string', type: 'string', indexed: false },
      { name: 'val', internalType: 'string', type: 'string', indexed: false },
    ],
    name: 'log_named_string',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'string', type: 'string', indexed: false },
      { name: 'val', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'log_named_uint',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: '', internalType: 'string', type: 'string', indexed: false },
    ],
    name: 'log_string',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: '', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'log_uint',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: '', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'logs',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TronTxReader
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const tronTxReaderAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'tronLightClient',
    outputs: [
      { name: '', internalType: 'contract TronLightClient', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'txNullifierSet',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// UntronController
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const untronControllerAbi = [
  {
    type: 'constructor',
    inputs: [{ name: 'create2Prefix', internalType: 'bytes1', type: 'bytes1' }],
    stateMutability: 'nonpayable',
  },
  { type: 'receive', stateMutability: 'payable' },
  {
    type: 'function',
    inputs: [
      { name: 'token', internalType: 'address', type: 'address' },
      { name: 'bridger', internalType: 'address', type: 'address' },
      { name: 'inAmount', internalType: 'uint256', type: 'uint256' },
      { name: 'outAmount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'bridge',
    outputs: [],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [{ name: 'salt', internalType: 'bytes32', type: 'bytes32' }],
    name: 'deployReceiver',
    outputs: [
      { name: 'receiver', internalType: 'address payable', type: 'address' },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'executor',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '', internalType: 'address', type: 'address' }],
    name: 'isBridgerAllowed',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
    ],
    name: 'payloadFor',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'controller', internalType: 'address', type: 'address' },
      { name: 'salt', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'predictReceiverAddress',
    outputs: [{ name: 'predicted', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'token', internalType: 'address', type: 'address' },
      { name: 'receiverSalts', internalType: 'bytes32[]', type: 'bytes32[]' },
      { name: 'amounts', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    name: 'pullFromReceivers',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'receiverBytecode',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [
      { name: '_bridger', internalType: 'address', type: 'address' },
      { name: '_allowed', internalType: 'bool', type: 'bool' },
    ],
    name: 'setBridgerAllowed',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '_executor', internalType: 'address', type: 'address' }],
    name: 'setExecutor',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'setOwner',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_token', internalType: 'address', type: 'address' },
      { name: '_bridger', internalType: 'address', type: 'address' },
      { name: '_payload', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'setPayload',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'token', internalType: 'address', type: 'address' },
      { name: 'recipient', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFromController',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'bridger',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'allowed', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'BridgerAllowed',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'token',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'recipient',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ControllerTransfer',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newExecutor',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'ExecutorChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnerChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'token',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'bridger',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'PayloadSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'token',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'recipient',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ReceiverCalled',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'receiver',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'salt',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'ReceiverDeployed',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'token',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'inAmount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'outAmount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'bridger',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'TokensBridged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'token',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'totalAmount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'TokensPulled',
  },
  { type: 'error', inputs: [], name: 'BridgerNotAllowed' },
  { type: 'error', inputs: [], name: 'IncorrectSweepAmount' },
  { type: 'error', inputs: [], name: 'LengthMismatch' },
  { type: 'error', inputs: [], name: 'OnlyExecutor' },
  { type: 'error', inputs: [], name: 'OnlyOwner' },
  { type: 'error', inputs: [], name: 'OutAmountMismatch' },
  { type: 'error', inputs: [], name: 'RouteNotSet' },
  { type: 'error', inputs: [], name: 'ZeroOwnerAddress' },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// UntronManager
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const untronManagerAbi = [
  {
    type: 'constructor',
    inputs: [
      { name: 'controllerAddress', internalType: 'bytes20', type: 'bytes20' },
      { name: 'create2Prefix', internalType: 'bytes1', type: 'bytes1' },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'CONTROLLER_ADDRESS',
    outputs: [{ name: '', internalType: 'bytes20', type: 'bytes20' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'salt', internalType: 'bytes32', type: 'bytes32' }],
    name: 'deployReceiver',
    outputs: [
      { name: 'receiver', internalType: 'address payable', type: 'address' },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'controller', internalType: 'address', type: 'address' },
      { name: 'salt', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'predictReceiverAddress',
    outputs: [{ name: 'predicted', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'receiverBytecode',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [],
    name: 'tronLightClient',
    outputs: [
      { name: '', internalType: 'contract TronLightClient', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'txNullifierSet',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// UntronReceiver
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const untronReceiverAbi = [
  { type: 'constructor', inputs: [], stateMutability: 'nonpayable' },
  { type: 'receive', stateMutability: 'payable' },
  {
    type: 'function',
    inputs: [
      { name: 'token', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'recipient', internalType: 'address payable', type: 'address' },
    ],
    name: 'onControllerCall',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  { type: 'error', inputs: [], name: 'NotController' },
] as const
