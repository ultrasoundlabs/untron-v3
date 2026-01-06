"use strict";
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __commonJS = (cb, mod) => function __require() {
  return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));

// node_modules/.pnpm/call-me-maybe@1.0.2/node_modules/call-me-maybe/src/next.js
var require_next = __commonJS({
  "node_modules/.pnpm/call-me-maybe@1.0.2/node_modules/call-me-maybe/src/next.js"(exports2, module2) {
    "use strict";
    function makeNext() {
      if (typeof process === "object" && typeof process.nextTick === "function") {
        return process.nextTick;
      } else if (typeof setImmediate === "function") {
        return setImmediate;
      } else {
        return function next(f) {
          setTimeout(f, 0);
        };
      }
    }
    module2.exports = makeNext();
  }
});

// node_modules/.pnpm/call-me-maybe@1.0.2/node_modules/call-me-maybe/src/maybe.js
var require_maybe = __commonJS({
  "node_modules/.pnpm/call-me-maybe@1.0.2/node_modules/call-me-maybe/src/maybe.js"(exports2, module2) {
    "use strict";
    var next = require_next();
    module2.exports = function maybe(cb, promise) {
      if (cb) {
        promise.then(function(result) {
          next(function() {
            cb(null, result);
          });
        }, function(err) {
          next(function() {
            cb(err);
          });
        });
        return void 0;
      } else {
        return promise;
      }
    };
  }
});

// node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/utils.js
var require_utils = __commonJS({
  "node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/utils.js"(exports2, module2) {
    var DebounceTimers = class {
      constructor(cb, defaultDelay) {
        this.cb = cb;
        this.delay = defaultDelay;
        this.timers = {};
        this.pausers = {};
      }
      setDelay(delay) {
        if (delay >= 0)
          this.delay = delay;
      }
      pause(key) {
        this.pausers[key] = this.pausers[key] || 0;
        this.pausers[key]++;
      }
      unpause(key) {
        var count = this.pausers[key] || 0;
        if (count > 0)
          count--;
        this.pausers[key] = count;
      }
      unpauseAndTime(key) {
        this.unpause(key);
        this.time(key);
      }
      time(key) {
        var self = this;
        var timers = this.timers;
        var timer = this.timers[key];
        if (this.pausers[key] > 0)
          return;
        if (timer)
          clearTimeout(timer);
        timers[key] = setTimeout(function onTimer() {
          self.cb(key);
          delete timers[key];
        }, self.delay);
      }
    };
    var ERR_INVALID_ARG_TYPE = class extends TypeError {
      constructor(name, expected, actual) {
        const type = name.includes(".") ? "property" : "argument";
        let msg = `The "${name}" ${type} ${determiner} ${expected}`;
      }
    };
    function assertIsObject(value, name, types = "Object") {
      if (value !== void 0 && (value === null || typeof value !== "object" || Array.isArray(value))) {
        const err = new ERR_INVALID_ARG_TYPE(name, types, value);
        Error.captureStackTrace(err, assertIsObject);
        throw err;
      }
    }
    module2.exports = {
      ERR_INVALID_ARG_TYPE,
      assertIsObject,
      DebounceTimers
    };
  }
});

// node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/request-options.js
var require_request_options = __commonJS({
  "node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/request-options.js"(exports2, module2) {
    var { assertIsObject } = require_utils();
    function initializeOptions(options) {
      assertIsObject(options, "options");
      options = Object.assign({}, options);
      options.allowHalfOpen = true;
      options.rejectUnauthorized = false;
      assertIsObject(options.settings, "options.settings");
      options.settings = Object.assign({}, options.settings);
      options.Http1IncomingMessage = options.Http1IncomingMessage || this.http.IncomingMessage;
      options.Http1ServerResponse = options.Http1ServerResponse || this.http.ServerResponse;
      options.Http2ServerRequest = options.Http2ServerRequest || (this.http2 || {}).Http2ServerRequest;
      options.Http2ServerResponse = options.Http2ServerResponse || (this.http2 || {}).Http2ServerResponse;
      return options;
    }
    function initializeTLSOptions(options, servername) {
      options = initializeOptions.call(this, options);
      var ALPNProtocols = options.ALPNProtocols = [];
      if (this.http2Support)
        ALPNProtocols.push("h2");
      if (options.allowHTTP1 == true || !this.http2Support)
        ALPNProtocols.push("http/1.1");
      if (servername !== void 0 && options.servername === void 0)
        options.servername = servername;
      return options;
    }
    module2.exports = {
      initializeTLSOptions
    };
  }
});

// node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/request.js
var require_request = __commonJS({
  "node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/request.js"(exports2, module2) {
    var { URL: URL2 } = require("url");
    var { EventEmitter } = require("events");
    var _extend = require("util")._extend;
    var { DebounceTimers, assertIsObject, ERR_INVALID_ARG_TYPE } = require_utils();
    var { initializeTLSOptions } = require_request_options();
    var http = require("http");
    var https = require("https");
    var { Stream } = require("stream");
    function addFunctions(container, obj) {
      const proto = obj.prototype;
      Object.keys(proto).forEach((name) => {
        if (container.indexOf(name) != -1)
          return;
        if (name.indexOf("_") != 0 && typeof proto[name] == "function") {
          container.push(name);
        }
      });
    }
    var STUBBED_METHODS_NAME = [];
    addFunctions(STUBBED_METHODS_NAME, http.ClientRequest);
    addFunctions(STUBBED_METHODS_NAME, http.OutgoingMessage);
    addFunctions(STUBBED_METHODS_NAME, EventEmitter);
    addFunctions(STUBBED_METHODS_NAME, Stream);
    var PROPERTIES_TO_PROXY = [
      "httpVersionMajor",
      "httpVersionMinor",
      "httpVersion"
    ];
    var HEADERS_TO_REMOVE = ["host", "connection"];
    var $stubs = /* @__PURE__ */ Symbol("stubs");
    function ClientRequest() {
      this.http2Mimic = true;
      this[$stubs] = [];
      for (var i = 0; i < STUBBED_METHODS_NAME.length; i++) {
        let name = STUBBED_METHODS_NAME[i];
        if (!ClientRequest.prototype[name]) {
          this[name] = function method() {
            return this.genericStubber(name, arguments);
          }.bind(this);
        }
      }
      var requestOptions, cb, url, args;
      const isInternal = arguments[0] instanceof RequestInternalEnforce;
      var isInternalMethod, isInternalProtocol;
      if (isInternal) {
        const enforceOptions = arguments[0];
        if (enforceOptions.method)
          isInternalMethod = enforceOptions.method;
        if (enforceOptions.protocol)
          isInternalProtocol = enforceOptions.protocol;
      }
      if (isInternal) {
        args = arguments[0].args;
      } else {
        args = arguments;
      }
      if (args[2] != void 0) {
        url = args[0];
        requestOptions = args[1];
        cb = args[2];
      } else if (args[1] == void 0) {
        requestOptions = args[0];
      } else {
        requestOptions = args[0];
        cb = args[1];
      }
      cb = cb || function dummy() {
      };
      if (typeof requestOptions === "string") {
        requestOptions = urlToOptions(new URL2(requestOptions));
        if (!requestOptions.hostname) {
          throw new Error("Unable to determine the domain name");
        }
      } else {
        if (url) {
          requestOptions = _extend(urlToOptions(new URL2(url)), requestOptions);
        } else {
          requestOptions = _extend({}, requestOptions);
        }
      }
      if (isInternalProtocol != isInternalProtocol) {
        requestOptions.protocol = isInternalProtocol;
      }
      if (requestOptions.protocol == "https:" && !requestOptions.port && requestOptions.port != 0)
        requestOptions.port = 443;
      if (!requestOptions.port && requestOptions.port != 0)
        requestOptions.port = 80;
      if (isInternalMethod) {
        requestOptions.method = isInternalMethod;
      } else if (!requestOptions.method)
        requestOptions.method = "GET";
      requestOptions.method = requestOptions.method.toUpperCase();
      const requestManager = requestOptions.requestManager || this.getGlobalManager(requestOptions);
      requestManager.handleClientRequest(this, requestOptions, cb);
    }
    ClientRequest.prototype = {
      getGlobalManager(options) {
        if (options.agent)
          return options.agent.protocol == "https:" ? HttpsRequest.globalManager : HttpRequest.globalManager;
        else
          return HttpRequestManager.globalManager;
      },
      genericStubber(method, args) {
        if (this[$stubs]) {
          this[$stubs].push([method, args]);
          return true;
        } else
          return this[method](...arguments);
      },
      on(eventName, cb) {
        if (eventName == "response") {
          if (!cb.http2Safe) {
            eventName = "http1.response";
            arguments[0] = eventName;
          }
        }
        if (this._on) {
          this._on(...arguments);
        } else
          this.genericStubber("on", arguments);
      },
      once(eventName, cb) {
        if (eventName == "response") {
          if (!cb.http2Safe) {
            eventName = "http1.response";
          }
        }
        if (this._once) {
          this._once(...arguments);
        } else
          this.genericStubber("once", arguments);
      },
      emitError(error) {
        if (this[$stubs]) {
          this[$stubs].forEach(([method, args]) => {
            if ((method === "on" || method === "once") && args[0] === "error") {
              args[1](error);
            }
          });
        } else
          return this.emit("error", error);
      },
      take(stream) {
        for (var i = 0; i < STUBBED_METHODS_NAME.length; i++) {
          let name = STUBBED_METHODS_NAME[i];
          if (stream[name]) {
            this[name] = stream[name].bind(stream);
          }
        }
        this._on = stream.on.bind(stream);
        this._once = stream.once.bind(stream);
        this.proxyProps(stream);
        for (let i2 = 0; i2 < this[$stubs].length; i2++) {
          var stub = this[$stubs][i2];
          stream[stub[0]](...stub[1]);
        }
        this[$stubs] = null;
      },
      proxyProps(http2Stream) {
        function getter() {
          return http2Stream[this];
        }
        function setter(value) {
          http2Stream[this] = value;
        }
        const notToProxy = ["on", "_on", "_once", "once", "http2Mimic"].concat(STUBBED_METHODS_NAME);
        const keys = Object.keys(this);
        const keysToProxy = [].concat(PROPERTIES_TO_PROXY);
        keys.forEach(function whichProxyKeys(key) {
          if (notToProxy.indexOf(key) == -1 && keysToProxy.indexOf(key) == -1) {
            keysToProxy.push(key);
          }
        });
        const properties = Object.getOwnPropertyDescriptors(http2Stream);
        for (var i = 0; i < keysToProxy.length; i++) {
          let name = keysToProxy[i];
          const propConfig = properties[name];
          let shouldCopyValue;
          if (!propConfig)
            shouldCopyValue = true;
          if (propConfig && (propConfig.writable || propConfig))
            shouldCopyValue = true;
          if (shouldCopyValue)
            http2Stream[name] = this[name];
          Object.defineProperty(this, name, {
            get: getter.bind(name),
            set: setter.bind(name)
          });
        }
      }
    };
    var HttpRequestManager = class _HttpRequestManager extends EventEmitter {
      constructor(options) {
        super();
        this.httpsAgent = https.globalAgent;
        this.httpAgent = http.globalAgent;
        this.init(options);
      }
      log() {
      }
      init(options) {
        options = options || {};
        this.http2Clients = {};
        this.cachedHTTP1Result = {};
        this.setModules();
        this.http2Debouncer = new DebounceTimers(function stopConnection(key) {
          this.log("stopping ", key);
          var foundConnection = this.http2Clients[key];
          if (foundConnection) {
            this.removeHttp2Client(key, foundConnection);
          }
        }.bind(this), 1e3);
        this.keepH1IdentificationCacheFor = options.keepH1IdentificationCacheFor || 3e4;
        this.http2Debouncer.setDelay(options.keepH2ConnectionFor);
        if (options.useHttp) {
          this.enforceProtocol = "http:";
        } else if (options.useHttps) {
          this.enforceProtocol = "https:";
        }
      }
      setModules() {
        this["http"] = require("http");
        this["https"] = require("https");
        this["tls"] = require("tls");
        this["net"] = require("net");
        this.http2Support = false;
        try {
          this["http2"] = require("http2");
          this.http2Support = true;
        } catch (err) {
        }
      }
      handleClientRequest(clientRequest, requestOptions, cb) {
        const requestManager = this;
        const clientKey = requestManager.getClientKey(requestOptions);
        if (requestManager.hasCachedConnection(clientKey)) {
          const socket = requestManager.getHttp2Client(clientKey);
          const connectionOptions = {
            createConnection() {
              return socket;
            }
          };
          process.nextTick(function onMakeRequest() {
            requestManager.makeRequest(clientRequest, clientKey, requestOptions, cb, connectionOptions);
          }.bind(requestManager));
        } else
          requestManager.holdConnectionToIdentification(clientKey, requestOptions, function onIdentification(error, connectionOptions) {
            if (error) {
              clientRequest.emitError(error);
              return;
            }
            requestManager.makeRequest(clientRequest, clientKey, requestOptions, cb, connectionOptions);
          }.bind(requestManager));
      }
      getClientKey(url) {
        return `${url.protocol || this.enforceProtocol}${url.servername || url.host || url.hostname}:${url.port}`;
      }
      getHttp2Client(clientKey) {
        return this.http2Clients[clientKey];
      }
      setHttp2Client(clientKey, client) {
        const httpManager = this;
        const prevClient = httpManager.http2Clients[clientKey];
        if (prevClient)
          httpManager.removeHttp2Client(clientKey, prevClient);
        httpManager.http2Clients[clientKey] = client;
        function closeClient() {
          httpManager.removeHttp2Client(clientKey, client);
        }
        client.on("close", closeClient);
        client.on("goaway", closeClient);
        client.on("error", closeClient);
        client.on("frameError", closeClient);
        client.on("timeout", closeClient);
      }
      removeHttp2Client(clientKey, client) {
        try {
          delete this.http2Clients[clientKey];
          if (!client.closed) {
            client.close();
          }
        } catch (err) {
        }
        client.removeAllListeners("close");
        client.removeAllListeners("error");
        client.removeAllListeners("frameError");
        client.removeAllListeners("timeout");
      }
      request(url, options, cb) {
        var args = new RequestInternalEnforce(arguments);
        if (this.enforceProtocol) {
          args.protocol = this.enforceProtocol;
        }
        return new ClientRequest(args);
      }
      get() {
        var args = new RequestInternalEnforce(arguments);
        args.method = "GET";
        var request = this.request(args);
        request.end();
        return request;
      }
      hasCachedConnection(clientKey) {
        const http2Client = this.getHttp2Client(clientKey);
        if (http2Client) {
          return true;
        }
        return this.cachedHTTP1Result[clientKey] + this.keepH1IdentificationCacheFor < Date.now();
      }
      makeRequest(inStream, clientKey, requestOptions, cb, connectionOptions) {
        const http2Client = this.getHttp2Client(clientKey);
        if (http2Client) {
          return this.makeHttp2Request(clientKey, inStream, http2Client, Object.assign(connectionOptions || {}, requestOptions), cb);
        }
        if (!requestOptions.agent) {
          if (requestOptions.protocol == "https:")
            requestOptions.agent = this.httpsAgent;
          else
            requestOptions.agent = this.httpAgent;
        }
        return this.makeHttpRequest(clientKey, inStream, requestOptions, cb, connectionOptions);
      }
      holdConnectionToIdentification(clientKey, requestOptions, cb) {
        const topic = `identify-${clientKey}`;
        if (this._events[topic])
          this.once(topic, cb);
        else {
          this.once(topic, function letKnowThereIsAnEvent() {
          });
          const socket = this.identifyConnection(requestOptions, function onIdentify(error, type) {
            if (error) {
              return cb(error);
            }
            var options = {
              createConnection() {
                return socket;
              }
            };
            if (type == "h2" && this.http2Support) {
              var http2Client = this.http2.connect(requestOptions, options);
              this.setHttp2Client(clientKey, http2Client);
            } else {
              this.cachedHTTP1Result[clientKey] = Date.now();
            }
            cb(null, options);
            this.emit(topic, options);
          }.bind(this));
        }
      }
      makeHttpRequest(clientKey, inStream, options, cb, connectionOptions) {
        if (options instanceof URL2)
          options = urlToOptions(options);
        const h1op = _extend({}, options);
        if (connectionOptions)
          h1op.createConnection = connectionOptions.createConnection;
        const requestModule = h1op.protocol == "https:" ? this.https : this.http;
        const req = requestModule.request(h1op, cb);
        inStream.take(req);
        inStream._on("response", function onHttp1Response(v) {
          this.emit("http1.response", v);
        });
      }
      makeHttp2Request(clientKey, inStream, http2Client, requestOptions, cb) {
        var http2Debouncer = this.http2Debouncer;
        http2Debouncer.pause(clientKey);
        var headers = _extend({}, requestOptions.headers || {});
        if (requestOptions.method)
          headers[":method"] = requestOptions.method;
        if (requestOptions.path)
          headers[":path"] = requestOptions.path;
        Object.keys(headers).forEach((key) => {
          if (HEADERS_TO_REMOVE.indexOf((key + "").toLowerCase()) != -1) {
            delete headers[key];
          }
        });
        requestOptions.headers = headers;
        var req = http2Client.request(
          headers
        );
        inStream.emit("socket", requestOptions.createConnection());
        let maxContentLength;
        let currentContent = 0;
        req.on("data", function onData(data) {
          currentContent += data.length;
          if (currentContent >= maxContentLength)
            http2Debouncer.unpauseAndTime(clientKey);
        });
        inStream.take(req);
        function onResponse(headers2) {
          maxContentLength = parseInt(headers2["content-length"]);
          if (maxContentLength < 0)
            this.http2Debouncer.unpauseAndTime(clientKey);
          _HttpRequestManager.httpCompatibleResponse(req, requestOptions, headers2);
          inStream.emit("http1.response", req);
          if (cb)
            cb(req);
        }
        onResponse.http2Safe = true;
        req.once("response", onResponse.bind(this));
      }
      static httpCompatibleResponse(res, requestOptions, headers) {
        res.httpVersion = "2.0";
        res.rawHeaders = headers;
        res.headers = headers;
        res.statusCode = headers[":status"];
        delete headers[":status"];
      }
      identifyConnection(requestOptions, cb) {
        var socket = this.connect(requestOptions, { allowHTTP1: true }, function onConnect() {
          socket.removeListener("error", cb);
          if (socket.alpnProtocol == "h2") {
            cb(null, "h2");
          } else {
            socket.end();
            cb(null, "h1");
          }
        });
        socket.on("error", cb);
        return socket;
      }
      connect(authority, options, listener) {
        if (typeof options === "function") {
          listener = options;
          options = void 0;
        }
        assertIsObject(options, "options");
        options = Object.assign({}, options);
        if (typeof authority === "string")
          authority = new URL2(authority);
        assertIsObject(authority, "authority", ["string", "Object", "URL"]);
        var protocol = authority.protocol || options.protocol || (this.enforceProtocol != "detect" ? this.enforceProtocol : null) || "http:";
        var port = "" + (authority.port !== "" ? authority.port : authority.protocol === "http:" ? 80 : 443);
        var host = authority.hostname || authority.host || "localhost";
        var socket;
        if (typeof options.createConnection === "function") {
          socket = options.createConnection(authority, options);
        } else {
          switch (protocol) {
            case "http:":
              socket = this.net.connect(port, host, listener);
              break;
            case "https:":
              socket = this.tls.connect(port, host, initializeTLSOptions.call(this, options, host), listener);
              break;
            default:
              throw new Error("Not supprted" + protocol);
          }
        }
        return socket;
      }
    };
    function urlToOptions(url) {
      var options = {
        protocol: url.protocol,
        hostname: url.hostname,
        hash: url.hash,
        search: url.search,
        pathname: url.pathname,
        path: `${url.pathname}${url.search}`,
        href: url.href
      };
      if (url.port !== "") {
        options.port = Number(url.port);
      }
      if (url.username || url.password) {
        options.auth = `${url.username}:${url.password}`;
      }
      return options;
    }
    var RequestInternalEnforce = class _RequestInternalEnforce {
      constructor(args) {
        if (args[0] instanceof _RequestInternalEnforce) {
          return args[0];
        }
        this.args = args;
        this.method = null;
        this.protocol = null;
      }
    };
    var HttpsRequest = class extends HttpRequestManager {
      constructor() {
        super(...arguments);
        this.Agent = https.Agent;
        this.globalAgent = https.globalAgent;
        this.enforceProtocol = "https:";
      }
    };
    var httpsRequestSinglton = new HttpsRequest();
    HttpsRequest.globalManager = httpsRequestSinglton;
    HttpsRequest.Manager = HttpsRequest;
    var HttpRequest = class extends HttpRequestManager {
      constructor() {
        super(...arguments);
        this.Agent = http.Agent;
        this.globalAgent = http.globalAgent;
        this.enforceProtocol = "http:";
      }
    };
    var httpRequestSinglton = new HttpRequest();
    HttpRequest.globalManager = httpRequestSinglton;
    HttpRequest.Manager = HttpRequest;
    var singeltonHttpManager = new HttpRequestManager();
    singeltonHttpManager.enforceProtocol = "detect";
    HttpRequestManager.globalManager = singeltonHttpManager;
    module2.exports = {
      HttpRequest,
      HttpsRequest,
      HTTP2OutgoingMessage: ClientRequest,
      ClientRequest,
      HttpRequestManager
    };
  }
});

// node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/http.js
var require_http = __commonJS({
  "node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/http.js"(exports2, module2) {
    var {
      HttpRequest,
      ClientRequest
    } = require_request();
    var globalManager = HttpRequest.globalManager;
    var request = globalManager.request.bind(globalManager);
    var get = globalManager.get.bind(globalManager);
    var http = Object.assign({}, require("http"));
    module2.exports = Object.assign(http, {
      ClientRequest,
      globalManager,
      request,
      get
    });
  }
});

// node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/https.js
var require_https = __commonJS({
  "node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/https.js"(exports2, module2) {
    var {
      HttpsRequest,
      ClientRequest
    } = require_request();
    var globalManager = HttpsRequest.globalManager;
    var request = globalManager.request.bind(globalManager);
    var get = globalManager.get.bind(globalManager);
    var https = Object.assign({}, require("https"));
    module2.exports = Object.assign(https, {
      ClientRequest,
      globalManager,
      request,
      get
    });
  }
});

// node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/index.js
var require_lib = __commonJS({
  "node_modules/.pnpm/http2-client@1.3.5/node_modules/http2-client/lib/index.js"(exports2, module2) {
    var {
      HttpRequestManager,
      HTTP2OutgoingMessage,
      ClientRequest
    } = require_request();
    var http = require_http();
    var https = require_https();
    var autoDetectManager = new HttpRequestManager();
    HttpRequestManager.globalManager = autoDetectManager;
    var request = autoDetectManager.request.bind(autoDetectManager);
    var get = autoDetectManager.get.bind(autoDetectManager);
    module2.exports = {
      HTTP2OutgoingMessage,
      ClientRequest,
      globalManager: HttpRequestManager.globalManager,
      request,
      get,
      http,
      https
    };
  }
});

// node_modules/.pnpm/node-fetch-h2@2.3.0/node_modules/node-fetch-h2/lib/index.js
var require_lib2 = __commonJS({
  "node_modules/.pnpm/node-fetch-h2@2.3.0/node_modules/node-fetch-h2/lib/index.js"(exports2, module2) {
    "use strict";
    Object.defineProperty(exports2, "__esModule", { value: true });
    function _interopDefault(ex) {
      return ex && typeof ex === "object" && "default" in ex ? ex["default"] : ex;
    }
    var Stream = _interopDefault(require("stream"));
    var http = _interopDefault(require("http"));
    var Url = _interopDefault(require("url"));
    var h2 = _interopDefault(require_lib());
    var zlib = _interopDefault(require("zlib"));
    var BUFFER = /* @__PURE__ */ Symbol("buffer");
    var TYPE = /* @__PURE__ */ Symbol("type");
    var Blob = class _Blob {
      constructor() {
        this[TYPE] = "";
        const blobParts = arguments[0];
        const options = arguments[1];
        const buffers = [];
        if (blobParts) {
          const a = blobParts;
          const length = Number(a.length);
          for (let i = 0; i < length; i++) {
            const element = a[i];
            let buffer;
            if (element instanceof Buffer) {
              buffer = element;
            } else if (ArrayBuffer.isView(element)) {
              buffer = Buffer.from(element.buffer, element.byteOffset, element.byteLength);
            } else if (element instanceof ArrayBuffer) {
              buffer = Buffer.from(element);
            } else if (element instanceof _Blob) {
              buffer = element[BUFFER];
            } else {
              buffer = Buffer.from(typeof element === "string" ? element : String(element));
            }
            buffers.push(buffer);
          }
        }
        this[BUFFER] = Buffer.concat(buffers);
        let type = options && options.type !== void 0 && String(options.type).toLowerCase();
        if (type && !/[^\u0020-\u007E]/.test(type)) {
          this[TYPE] = type;
        }
      }
      get size() {
        return this[BUFFER].length;
      }
      get type() {
        return this[TYPE];
      }
      slice() {
        const size = this.size;
        const start = arguments[0];
        const end = arguments[1];
        let relativeStart, relativeEnd;
        if (start === void 0) {
          relativeStart = 0;
        } else if (start < 0) {
          relativeStart = Math.max(size + start, 0);
        } else {
          relativeStart = Math.min(start, size);
        }
        if (end === void 0) {
          relativeEnd = size;
        } else if (end < 0) {
          relativeEnd = Math.max(size + end, 0);
        } else {
          relativeEnd = Math.min(end, size);
        }
        const span = Math.max(relativeEnd - relativeStart, 0);
        const buffer = this[BUFFER];
        const slicedBuffer = buffer.slice(relativeStart, relativeStart + span);
        const blob = new _Blob([], { type: arguments[2] });
        blob[BUFFER] = slicedBuffer;
        return blob;
      }
    };
    Object.defineProperties(Blob.prototype, {
      size: { enumerable: true },
      type: { enumerable: true },
      slice: { enumerable: true }
    });
    Object.defineProperty(Blob.prototype, Symbol.toStringTag, {
      value: "Blob",
      writable: false,
      enumerable: false,
      configurable: true
    });
    function FetchError(message, type, systemError) {
      Error.call(this, message);
      this.message = message;
      this.type = type;
      if (systemError) {
        this.code = this.errno = systemError.code;
      }
      Error.captureStackTrace(this, this.constructor);
    }
    FetchError.prototype = Object.create(Error.prototype);
    FetchError.prototype.constructor = FetchError;
    FetchError.prototype.name = "FetchError";
    var convert;
    try {
      convert = require("encoding").convert;
    } catch (e) {
    }
    var INTERNALS = /* @__PURE__ */ Symbol("Body internals");
    var PassThrough = Stream.PassThrough;
    function Body(body) {
      var _this = this;
      var _ref = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : {}, _ref$size = _ref.size;
      let size = _ref$size === void 0 ? 0 : _ref$size;
      var _ref$timeout = _ref.timeout;
      let timeout = _ref$timeout === void 0 ? 0 : _ref$timeout;
      if (body == null) {
        body = null;
      } else if (isURLSearchParams(body)) {
        body = Buffer.from(body.toString());
      } else if (body instanceof Blob) {
        body = body[BUFFER];
      } else if (Buffer.isBuffer(body)) ;
      else if (Object.prototype.toString.call(body) === "[object ArrayBuffer]") {
        body = Buffer.from(body);
      } else if (ArrayBuffer.isView(body)) {
        body = Buffer.from(body.buffer, body.byteOffset, body.byteLength);
      } else if (body instanceof Stream) ;
      else {
        body = Buffer.from(String(body));
      }
      this[INTERNALS] = {
        body,
        disturbed: false,
        error: null
      };
      this.size = size;
      this.timeout = timeout;
      if (body instanceof Stream) {
        body.on("error", function(err) {
          const error = err.name === "AbortError" ? err : new FetchError(`Invalid response body while trying to fetch ${_this.url}: ${err.message}`, "system", err);
          _this[INTERNALS].error = error;
        });
      }
    }
    Body.prototype = {
      get body() {
        return this[INTERNALS].body;
      },
      get bodyUsed() {
        return this[INTERNALS].disturbed;
      },
      /**
       * Decode response as ArrayBuffer
       *
       * @return  Promise
       */
      arrayBuffer() {
        return consumeBody.call(this).then(function(buf) {
          return buf.buffer.slice(buf.byteOffset, buf.byteOffset + buf.byteLength);
        });
      },
      /**
       * Return raw response as Blob
       *
       * @return Promise
       */
      blob() {
        let ct = this.headers && this.headers.get("content-type") || "";
        return consumeBody.call(this).then(function(buf) {
          return Object.assign(
            // Prevent copying
            new Blob([], {
              type: ct.toLowerCase()
            }),
            {
              [BUFFER]: buf
            }
          );
        });
      },
      /**
       * Decode response as json
       *
       * @return  Promise
       */
      json() {
        var _this2 = this;
        return consumeBody.call(this).then(function(buffer) {
          try {
            return JSON.parse(buffer.toString());
          } catch (err) {
            return Body.Promise.reject(new FetchError(`invalid json response body at ${_this2.url} reason: ${err.message}`, "invalid-json"));
          }
        });
      },
      /**
       * Decode response as text
       *
       * @return  Promise
       */
      text() {
        return consumeBody.call(this).then(function(buffer) {
          return buffer.toString();
        });
      },
      /**
       * Decode response as buffer (non-spec api)
       *
       * @return  Promise
       */
      buffer() {
        return consumeBody.call(this);
      },
      /**
       * Decode response as text, while automatically detecting the encoding and
       * trying to decode to UTF-8 (non-spec api)
       *
       * @return  Promise
       */
      textConverted() {
        var _this3 = this;
        return consumeBody.call(this).then(function(buffer) {
          return convertBody(buffer, _this3.headers);
        });
      }
    };
    Object.defineProperties(Body.prototype, {
      body: { enumerable: true },
      bodyUsed: { enumerable: true },
      arrayBuffer: { enumerable: true },
      blob: { enumerable: true },
      json: { enumerable: true },
      text: { enumerable: true }
    });
    Body.mixIn = function(proto) {
      for (const name of Object.getOwnPropertyNames(Body.prototype)) {
        if (!(name in proto)) {
          const desc = Object.getOwnPropertyDescriptor(Body.prototype, name);
          Object.defineProperty(proto, name, desc);
        }
      }
    };
    function consumeBody() {
      var _this4 = this;
      if (this[INTERNALS].disturbed) {
        return Body.Promise.reject(new TypeError(`body used already for: ${this.url}`));
      }
      this[INTERNALS].disturbed = true;
      if (this[INTERNALS].error) {
        return Body.Promise.reject(this[INTERNALS].error);
      }
      if (this.body === null) {
        return Body.Promise.resolve(Buffer.alloc(0));
      }
      if (Buffer.isBuffer(this.body)) {
        return Body.Promise.resolve(this.body);
      }
      if (!(this.body instanceof Stream)) {
        return Body.Promise.resolve(Buffer.alloc(0));
      }
      let accum = [];
      let accumBytes = 0;
      let abort = false;
      return new Body.Promise(function(resolve, reject) {
        let resTimeout;
        if (_this4.timeout) {
          resTimeout = setTimeout(function() {
            abort = true;
            reject(new FetchError(`Response timeout while trying to fetch ${_this4.url} (over ${_this4.timeout}ms)`, "body-timeout"));
          }, _this4.timeout);
        }
        _this4.body.on("error", function(err) {
          if (err.name === "AbortError") {
            abort = true;
            reject(err);
          } else {
            reject(new FetchError(`Invalid response body while trying to fetch ${_this4.url}: ${err.message}`, "system", err));
          }
        });
        _this4.body.on("data", function(chunk) {
          if (abort || chunk === null) {
            return;
          }
          if (_this4.size && accumBytes + chunk.length > _this4.size) {
            abort = true;
            reject(new FetchError(`content size at ${_this4.url} over limit: ${_this4.size}`, "max-size"));
            return;
          }
          accumBytes += chunk.length;
          accum.push(chunk);
        });
        _this4.body.on("end", function() {
          if (abort) {
            return;
          }
          clearTimeout(resTimeout);
          try {
            resolve(Buffer.concat(accum));
          } catch (err) {
            reject(new FetchError(`Could not create Buffer from response body for ${_this4.url}: ${err.message}`, "system", err));
          }
        });
      });
    }
    function convertBody(buffer, headers) {
      if (typeof convert !== "function") {
        throw new Error("The package `encoding` must be installed to use the textConverted() function");
      }
      const ct = headers.get("content-type");
      let charset = "utf-8";
      let res, str;
      if (ct) {
        res = /charset=([^;]*)/i.exec(ct);
      }
      str = buffer.slice(0, 1024).toString();
      if (!res && str) {
        res = /<meta.+?charset=(['"])(.+?)\1/i.exec(str);
      }
      if (!res && str) {
        res = /<meta[\s]+?http-equiv=(['"])content-type\1[\s]+?content=(['"])(.+?)\2/i.exec(str);
        if (res) {
          res = /charset=(.*)/i.exec(res.pop());
        }
      }
      if (!res && str) {
        res = /<\?xml.+?encoding=(['"])(.+?)\1/i.exec(str);
      }
      if (res) {
        charset = res.pop();
        if (charset === "gb2312" || charset === "gbk") {
          charset = "gb18030";
        }
      }
      return convert(buffer, "UTF-8", charset).toString();
    }
    function isURLSearchParams(obj) {
      if (typeof obj !== "object" || typeof obj.append !== "function" || typeof obj.delete !== "function" || typeof obj.get !== "function" || typeof obj.getAll !== "function" || typeof obj.has !== "function" || typeof obj.set !== "function") {
        return false;
      }
      return obj.constructor.name === "URLSearchParams" || Object.prototype.toString.call(obj) === "[object URLSearchParams]" || typeof obj.sort === "function";
    }
    function clone(instance) {
      let p1, p2;
      let body = instance.body;
      if (instance.bodyUsed) {
        throw new Error("cannot clone body after it is used");
      }
      if (body instanceof Stream && typeof body.getBoundary !== "function") {
        p1 = new PassThrough();
        p2 = new PassThrough();
        body.pipe(p1);
        body.pipe(p2);
        instance[INTERNALS].body = p1;
        body = p2;
      }
      return body;
    }
    function extractContentType(body) {
      if (body === null) {
        return null;
      } else if (typeof body === "string") {
        return "text/plain;charset=UTF-8";
      } else if (isURLSearchParams(body)) {
        return "application/x-www-form-urlencoded;charset=UTF-8";
      } else if (body instanceof Blob) {
        return body.type || null;
      } else if (Buffer.isBuffer(body)) {
        return null;
      } else if (Object.prototype.toString.call(body) === "[object ArrayBuffer]") {
        return null;
      } else if (ArrayBuffer.isView(body)) {
        return null;
      } else if (typeof body.getBoundary === "function") {
        return `multipart/form-data;boundary=${body.getBoundary()}`;
      } else if (body instanceof Stream) {
        return null;
      } else {
        return "text/plain;charset=UTF-8";
      }
    }
    function getTotalBytes(instance) {
      const body = instance.body;
      if (body === null) {
        return 0;
      } else if (Buffer.isBuffer(body)) {
        return body.length;
      } else if (body && typeof body.getLengthSync === "function") {
        if (body._lengthRetrievers && body._lengthRetrievers.length == 0 || // 1.x
        body.hasKnownLength && body.hasKnownLength()) {
          return body.getLengthSync();
        }
        return null;
      } else {
        return null;
      }
    }
    function writeToStream(dest, instance) {
      const body = instance.body;
      if (body === null) {
        dest.end();
      } else if (Buffer.isBuffer(body)) {
        dest.write(body);
        dest.end();
      } else {
        body.pipe(dest);
      }
    }
    Body.Promise = global.Promise;
    var invalidTokenRegex = /[^\^_`a-zA-Z\-0-9!#$%&'*+.|~]/;
    var invalidHeaderCharRegex = /[^\t\x20-\x7e\x80-\xff]/;
    function validateName(name) {
      name = `${name}`;
      if (invalidTokenRegex.test(name)) {
        throw new TypeError(`${name} is not a legal HTTP header name`);
      }
    }
    function validateValue(value) {
      value = `${value}`;
      if (invalidHeaderCharRegex.test(value)) {
        throw new TypeError(`${value} is not a legal HTTP header value`);
      }
    }
    function find(map, name) {
      name = name.toLowerCase();
      for (const key in map) {
        if (key.toLowerCase() === name) {
          return key;
        }
      }
      return void 0;
    }
    var MAP = /* @__PURE__ */ Symbol("map");
    var Headers = class _Headers {
      /**
       * Headers class
       *
       * @param   Object  headers  Response headers
       * @return  Void
       */
      constructor() {
        let init = arguments.length > 0 && arguments[0] !== void 0 ? arguments[0] : void 0;
        this[MAP] = /* @__PURE__ */ Object.create(null);
        if (init instanceof _Headers) {
          const rawHeaders = init.raw();
          const headerNames = Object.keys(rawHeaders);
          for (const headerName of headerNames) {
            for (const value of rawHeaders[headerName]) {
              this.append(headerName, value);
            }
          }
          return;
        }
        if (init == null) ;
        else if (typeof init === "object") {
          const method = init[Symbol.iterator];
          if (method != null) {
            if (typeof method !== "function") {
              throw new TypeError("Header pairs must be iterable");
            }
            const pairs = [];
            for (const pair of init) {
              if (typeof pair !== "object" || typeof pair[Symbol.iterator] !== "function") {
                throw new TypeError("Each header pair must be iterable");
              }
              pairs.push(Array.from(pair));
            }
            for (const pair of pairs) {
              if (pair.length !== 2) {
                throw new TypeError("Each header pair must be a name/value tuple");
              }
              this.append(pair[0], pair[1]);
            }
          } else {
            for (const key of Object.keys(init)) {
              const value = init[key];
              this.append(key, value);
            }
          }
        } else {
          throw new TypeError("Provided initializer must be an object");
        }
      }
      /**
       * Return combined header value given name
       *
       * @param   String  name  Header name
       * @return  Mixed
       */
      get(name) {
        name = `${name}`;
        validateName(name);
        const key = find(this[MAP], name);
        if (key === void 0) {
          return null;
        }
        return this[MAP][key].join(", ");
      }
      /**
       * Iterate over all headers
       *
       * @param   Function  callback  Executed for each item with parameters (value, name, thisArg)
       * @param   Boolean   thisArg   `this` context for callback function
       * @return  Void
       */
      forEach(callback) {
        let thisArg = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : void 0;
        let pairs = getHeaders(this);
        let i = 0;
        while (i < pairs.length) {
          var _pairs$i = pairs[i];
          const name = _pairs$i[0], value = _pairs$i[1];
          callback.call(thisArg, value, name, this);
          pairs = getHeaders(this);
          i++;
        }
      }
      /**
       * Overwrite header values given name
       *
       * @param   String  name   Header name
       * @param   String  value  Header value
       * @return  Void
       */
      set(name, value) {
        name = `${name}`;
        value = `${value}`;
        validateName(name);
        validateValue(value);
        const key = find(this[MAP], name);
        this[MAP][key !== void 0 ? key : name] = [value];
      }
      /**
       * Append a value onto existing header
       *
       * @param   String  name   Header name
       * @param   String  value  Header value
       * @return  Void
       */
      append(name, value) {
        name = `${name}`;
        value = `${value}`;
        validateName(name);
        validateValue(value);
        const key = find(this[MAP], name);
        if (key !== void 0) {
          this[MAP][key].push(value);
        } else {
          this[MAP][name] = [value];
        }
      }
      /**
       * Check for header name existence
       *
       * @param   String   name  Header name
       * @return  Boolean
       */
      has(name) {
        name = `${name}`;
        validateName(name);
        return find(this[MAP], name) !== void 0;
      }
      /**
       * Delete all header values given name
       *
       * @param   String  name  Header name
       * @return  Void
       */
      delete(name) {
        name = `${name}`;
        validateName(name);
        const key = find(this[MAP], name);
        if (key !== void 0) {
          delete this[MAP][key];
        }
      }
      /**
       * Return raw headers (non-spec api)
       *
       * @return  Object
       */
      raw() {
        return this[MAP];
      }
      /**
       * Get an iterator on keys.
       *
       * @return  Iterator
       */
      keys() {
        return createHeadersIterator(this, "key");
      }
      /**
       * Get an iterator on values.
       *
       * @return  Iterator
       */
      values() {
        return createHeadersIterator(this, "value");
      }
      /**
       * Get an iterator on entries.
       *
       * This is the default iterator of the Headers object.
       *
       * @return  Iterator
       */
      [Symbol.iterator]() {
        return createHeadersIterator(this, "key+value");
      }
    };
    Headers.prototype.entries = Headers.prototype[Symbol.iterator];
    Object.defineProperty(Headers.prototype, Symbol.toStringTag, {
      value: "Headers",
      writable: false,
      enumerable: false,
      configurable: true
    });
    Object.defineProperties(Headers.prototype, {
      get: { enumerable: true },
      forEach: { enumerable: true },
      set: { enumerable: true },
      append: { enumerable: true },
      has: { enumerable: true },
      delete: { enumerable: true },
      keys: { enumerable: true },
      values: { enumerable: true },
      entries: { enumerable: true }
    });
    function getHeaders(headers) {
      let kind = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : "key+value";
      const keys = Object.keys(headers[MAP]).sort();
      return keys.map(kind === "key" ? function(k) {
        return k.toLowerCase();
      } : kind === "value" ? function(k) {
        return headers[MAP][k].join(", ");
      } : function(k) {
        return [k.toLowerCase(), headers[MAP][k].join(", ")];
      });
    }
    var INTERNAL = /* @__PURE__ */ Symbol("internal");
    function createHeadersIterator(target, kind) {
      const iterator = Object.create(HeadersIteratorPrototype);
      iterator[INTERNAL] = {
        target,
        kind,
        index: 0
      };
      return iterator;
    }
    var HeadersIteratorPrototype = Object.setPrototypeOf({
      next() {
        if (!this || Object.getPrototypeOf(this) !== HeadersIteratorPrototype) {
          throw new TypeError("Value of `this` is not a HeadersIterator");
        }
        var _INTERNAL = this[INTERNAL];
        const target = _INTERNAL.target, kind = _INTERNAL.kind, index = _INTERNAL.index;
        const values = getHeaders(target, kind);
        const len = values.length;
        if (index >= len) {
          return {
            value: void 0,
            done: true
          };
        }
        this[INTERNAL].index = index + 1;
        return {
          value: values[index],
          done: false
        };
      }
    }, Object.getPrototypeOf(Object.getPrototypeOf([][Symbol.iterator]())));
    Object.defineProperty(HeadersIteratorPrototype, Symbol.toStringTag, {
      value: "HeadersIterator",
      writable: false,
      enumerable: false,
      configurable: true
    });
    function exportNodeCompatibleHeaders(headers) {
      const obj = Object.assign({ __proto__: null }, headers[MAP]);
      const hostHeaderKey = find(headers[MAP], "Host");
      if (hostHeaderKey !== void 0) {
        obj[hostHeaderKey] = obj[hostHeaderKey][0];
      }
      return obj;
    }
    function createHeadersLenient(obj) {
      const headers = new Headers();
      for (const name of Object.keys(obj)) {
        if (invalidTokenRegex.test(name)) {
          continue;
        }
        if (Array.isArray(obj[name])) {
          for (const val of obj[name]) {
            if (invalidHeaderCharRegex.test(val)) {
              continue;
            }
            if (headers[MAP][name] === void 0) {
              headers[MAP][name] = [val];
            } else {
              headers[MAP][name].push(val);
            }
          }
        } else if (!invalidHeaderCharRegex.test(obj[name])) {
          headers[MAP][name] = [obj[name]];
        }
      }
      return headers;
    }
    var INTERNALS$1 = /* @__PURE__ */ Symbol("Response internals");
    var STATUS_CODES = http.STATUS_CODES;
    var Response = class _Response {
      constructor() {
        let body = arguments.length > 0 && arguments[0] !== void 0 ? arguments[0] : null;
        let opts = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : {};
        Body.call(this, body, opts);
        const status = opts.status || 200;
        const headers = new Headers(opts.headers);
        if (body != null && !headers.has("Content-Type")) {
          const contentType = extractContentType(body);
          if (contentType) {
            headers.append("Content-Type", contentType);
          }
        }
        this[INTERNALS$1] = {
          url: opts.url,
          status,
          statusText: opts.statusText || STATUS_CODES[status],
          headers
        };
      }
      get url() {
        return this[INTERNALS$1].url;
      }
      get status() {
        return this[INTERNALS$1].status;
      }
      /**
       * Convenience property representing if the request ended normally
       */
      get ok() {
        return this[INTERNALS$1].status >= 200 && this[INTERNALS$1].status < 300;
      }
      get statusText() {
        return this[INTERNALS$1].statusText;
      }
      get headers() {
        return this[INTERNALS$1].headers;
      }
      /**
       * Clone this response
       *
       * @return  Response
       */
      clone() {
        return new _Response(clone(this), {
          url: this.url,
          status: this.status,
          statusText: this.statusText,
          headers: this.headers,
          ok: this.ok
        });
      }
    };
    Body.mixIn(Response.prototype);
    Object.defineProperties(Response.prototype, {
      url: { enumerable: true },
      status: { enumerable: true },
      ok: { enumerable: true },
      statusText: { enumerable: true },
      headers: { enumerable: true },
      clone: { enumerable: true }
    });
    Object.defineProperty(Response.prototype, Symbol.toStringTag, {
      value: "Response",
      writable: false,
      enumerable: false,
      configurable: true
    });
    var INTERNALS$2 = /* @__PURE__ */ Symbol("Request internals");
    var parse_url = Url.parse;
    var format_url = Url.format;
    var streamDestructionSupported = "destroy" in Stream.Readable.prototype;
    function isRequest(input) {
      return typeof input === "object" && typeof input[INTERNALS$2] === "object";
    }
    function isAbortSignal(signal) {
      const proto = signal && typeof signal === "object" && Object.getPrototypeOf(signal);
      return !!(proto && proto.constructor.name === "AbortSignal");
    }
    var Request = class _Request {
      constructor(input) {
        let init = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : {};
        let parsedURL;
        if (!isRequest(input)) {
          if (input && input.href) {
            parsedURL = parse_url(input.href);
          } else {
            parsedURL = parse_url(`${input}`);
          }
          input = {};
        } else {
          parsedURL = parse_url(input.url);
        }
        let method = init.method || input.method || "GET";
        method = method.toUpperCase();
        if ((init.body != null || isRequest(input) && input.body !== null) && (method === "GET" || method === "HEAD")) {
          throw new TypeError("Request with GET/HEAD method cannot have body");
        }
        let inputBody = init.body != null ? init.body : isRequest(input) && input.body !== null ? clone(input) : null;
        Body.call(this, inputBody, {
          timeout: init.timeout || input.timeout || 0,
          size: init.size || input.size || 0
        });
        const headers = new Headers(init.headers || input.headers || {});
        if (inputBody != null && !headers.has("Content-Type")) {
          const contentType = extractContentType(inputBody);
          if (contentType) {
            headers.append("Content-Type", contentType);
          }
        }
        let signal = isRequest(input) ? input.signal : null;
        if ("signal" in init) signal = init.signal;
        if (signal != null && !isAbortSignal(signal)) {
          throw new TypeError("Expected signal to be an instanceof AbortSignal");
        }
        this[INTERNALS$2] = {
          method,
          redirect: init.redirect || input.redirect || "follow",
          headers,
          parsedURL,
          signal
        };
        this.follow = init.follow !== void 0 ? init.follow : input.follow !== void 0 ? input.follow : 20;
        this.compress = init.compress !== void 0 ? init.compress : input.compress !== void 0 ? input.compress : true;
        this.counter = init.counter || input.counter || 0;
        this.agent = init.agent || input.agent;
      }
      get method() {
        return this[INTERNALS$2].method;
      }
      get url() {
        return format_url(this[INTERNALS$2].parsedURL);
      }
      get headers() {
        return this[INTERNALS$2].headers;
      }
      get redirect() {
        return this[INTERNALS$2].redirect;
      }
      get signal() {
        return this[INTERNALS$2].signal;
      }
      /**
       * Clone this request
       *
       * @return  Request
       */
      clone() {
        return new _Request(this);
      }
    };
    Body.mixIn(Request.prototype);
    Object.defineProperty(Request.prototype, Symbol.toStringTag, {
      value: "Request",
      writable: false,
      enumerable: false,
      configurable: true
    });
    Object.defineProperties(Request.prototype, {
      method: { enumerable: true },
      url: { enumerable: true },
      headers: { enumerable: true },
      redirect: { enumerable: true },
      clone: { enumerable: true },
      signal: { enumerable: true }
    });
    function getNodeRequestOptions(request) {
      const parsedURL = request[INTERNALS$2].parsedURL;
      const headers = new Headers(request[INTERNALS$2].headers);
      if (!headers.has("Accept")) {
        headers.set("Accept", "*/*");
      }
      if (!parsedURL.protocol || !parsedURL.hostname) {
        throw new TypeError("Only absolute URLs are supported");
      }
      if (!/^https?:$/.test(parsedURL.protocol)) {
        throw new TypeError("Only HTTP(S) protocols are supported");
      }
      if (request.signal && request.body instanceof Stream.Readable && !streamDestructionSupported) {
        throw new Error("Cancellation of streamed requests with AbortSignal is not supported in node < 8");
      }
      let contentLengthValue = null;
      if (request.body == null && /^(POST|PUT)$/i.test(request.method)) {
        contentLengthValue = "0";
      }
      if (request.body != null) {
        const totalBytes = getTotalBytes(request);
        if (typeof totalBytes === "number") {
          contentLengthValue = String(totalBytes);
        }
      }
      if (contentLengthValue) {
        headers.set("Content-Length", contentLengthValue);
      }
      if (!headers.has("User-Agent")) {
        headers.set("User-Agent", "node-fetch/1.0 (+https://github.com/bitinn/node-fetch)");
      }
      if (request.compress && !headers.has("Accept-Encoding")) {
        headers.set("Accept-Encoding", "gzip,deflate");
      }
      if (!headers.has("Connection") && !request.agent) {
        headers.set("Connection", "close");
      }
      return Object.assign({}, parsedURL, {
        method: request.method,
        headers: exportNodeCompatibleHeaders(headers),
        agent: request.agent
      });
    }
    function AbortError(message) {
      Error.call(this, message);
      this.type = "aborted";
      this.message = message;
      Error.captureStackTrace(this, this.constructor);
    }
    AbortError.prototype = Object.create(Error.prototype);
    AbortError.prototype.constructor = AbortError;
    AbortError.prototype.name = "AbortError";
    var PassThrough$1 = Stream.PassThrough;
    var resolve_url = Url.resolve;
    function fetch2(url, opts) {
      if (!fetch2.Promise) {
        throw new Error("native promise missing, set fetch.Promise to your favorite alternative");
      }
      Body.Promise = fetch2.Promise;
      return new fetch2.Promise(function(resolve, reject) {
        const request = new Request(url, opts);
        const options = getNodeRequestOptions(request);
        const send = h2.request;
        const signal = request.signal;
        let response = null;
        const abort = function abort2() {
          let error = new AbortError("The user aborted a request.");
          reject(error);
          if (request.body && request.body instanceof Stream.Readable) {
            request.body.destroy(error);
          }
          if (!response || !response.body) return;
          response.body.emit("error", error);
        };
        if (signal && signal.aborted) {
          abort();
          return;
        }
        const abortAndFinalize = function abortAndFinalize2() {
          abort();
          finalize();
        };
        const req = send(options);
        let reqTimeout;
        if (signal) {
          signal.addEventListener("abort", abortAndFinalize);
        }
        function finalize() {
          req.abort();
          if (signal) signal.removeEventListener("abort", abortAndFinalize);
          clearTimeout(reqTimeout);
        }
        if (request.timeout) {
          req.once("socket", function(socket) {
            reqTimeout = setTimeout(function() {
              reject(new FetchError(`network timeout at: ${request.url}`, "request-timeout"));
              finalize();
            }, request.timeout);
          });
        }
        req.on("error", function(err) {
          reject(new FetchError(`request to ${request.url} failed, reason: ${err.message}`, "system", err));
          finalize();
        });
        req.on("response", function(res) {
          clearTimeout(reqTimeout);
          const headers = createHeadersLenient(res.headers);
          if (fetch2.isRedirect(res.statusCode)) {
            const location = headers.get("Location");
            const locationURL = location === null ? null : resolve_url(request.url, location);
            switch (request.redirect) {
              case "error":
                reject(new FetchError(`redirect mode is set to error: ${request.url}`, "no-redirect"));
                finalize();
                return;
              case "manual":
                if (locationURL !== null) {
                  try {
                    headers.set("Location", locationURL);
                  } catch (err) {
                    reject(err);
                  }
                }
                break;
              case "follow":
                if (locationURL === null) {
                  break;
                }
                if (request.counter >= request.follow) {
                  reject(new FetchError(`maximum redirect reached at: ${request.url}`, "max-redirect"));
                  finalize();
                  return;
                }
                const requestOpts = {
                  headers: new Headers(request.headers),
                  follow: request.follow,
                  counter: request.counter + 1,
                  agent: request.agent,
                  compress: request.compress,
                  method: request.method,
                  body: request.body,
                  signal: request.signal
                };
                if (res.statusCode !== 303 && request.body && getTotalBytes(request) === null) {
                  reject(new FetchError("Cannot follow redirect with body being a readable stream", "unsupported-redirect"));
                  finalize();
                  return;
                }
                if (res.statusCode === 303 || (res.statusCode === 301 || res.statusCode === 302) && request.method === "POST") {
                  requestOpts.method = "GET";
                  requestOpts.body = void 0;
                  requestOpts.headers.delete("content-length");
                }
                resolve(fetch2(new Request(locationURL, requestOpts)));
                finalize();
                return;
            }
          }
          res.once("end", function() {
            if (signal) signal.removeEventListener("abort", abortAndFinalize);
          });
          let body = res.pipe(new PassThrough$1());
          const response_options = {
            url: request.url,
            status: res.statusCode,
            statusText: res.statusMessage,
            headers,
            size: request.size,
            timeout: request.timeout
          };
          const codings = headers.get("Content-Encoding");
          if (!request.compress || request.method === "HEAD" || codings === null || res.statusCode === 204 || res.statusCode === 304) {
            response = new Response(body, response_options);
            resolve(response);
            return;
          }
          const zlibOptions = {
            flush: zlib.Z_SYNC_FLUSH,
            finishFlush: zlib.Z_SYNC_FLUSH
          };
          if (codings == "gzip" || codings == "x-gzip") {
            body = body.pipe(zlib.createGunzip(zlibOptions));
            response = new Response(body, response_options);
            resolve(response);
            return;
          }
          if (codings == "deflate" || codings == "x-deflate") {
            const raw = res.pipe(new PassThrough$1());
            raw.once("data", function(chunk) {
              if ((chunk[0] & 15) === 8) {
                body = body.pipe(zlib.createInflate());
              } else {
                body = body.pipe(zlib.createInflateRaw());
              }
              response = new Response(body, response_options);
              resolve(response);
            });
            return;
          }
          response = new Response(body, response_options);
          resolve(response);
        });
        writeToStream(req, request);
      });
    }
    fetch2.isRedirect = function(code) {
      return code === 301 || code === 302 || code === 303 || code === 307 || code === 308;
    };
    fetch2.Promise = global.Promise;
    module2.exports = exports2 = fetch2;
    Object.defineProperty(exports2, "__esModule", { value: true });
    exports2.default = exports2;
    exports2.Headers = Headers;
    exports2.Request = Request;
    exports2.Response = Response;
    exports2.FetchError = FetchError;
  }
});

// node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/PlainValue-ec8e588e.js
var require_PlainValue_ec8e588e = __commonJS({
  "node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/PlainValue-ec8e588e.js"(exports2) {
    "use strict";
    var Char = {
      ANCHOR: "&",
      COMMENT: "#",
      TAG: "!",
      DIRECTIVES_END: "-",
      DOCUMENT_END: "."
    };
    var Type = {
      ALIAS: "ALIAS",
      BLANK_LINE: "BLANK_LINE",
      BLOCK_FOLDED: "BLOCK_FOLDED",
      BLOCK_LITERAL: "BLOCK_LITERAL",
      COMMENT: "COMMENT",
      DIRECTIVE: "DIRECTIVE",
      DOCUMENT: "DOCUMENT",
      FLOW_MAP: "FLOW_MAP",
      FLOW_SEQ: "FLOW_SEQ",
      MAP: "MAP",
      MAP_KEY: "MAP_KEY",
      MAP_VALUE: "MAP_VALUE",
      PLAIN: "PLAIN",
      QUOTE_DOUBLE: "QUOTE_DOUBLE",
      QUOTE_SINGLE: "QUOTE_SINGLE",
      SEQ: "SEQ",
      SEQ_ITEM: "SEQ_ITEM"
    };
    var defaultTagPrefix = "tag:yaml.org,2002:";
    var defaultTags = {
      MAP: "tag:yaml.org,2002:map",
      SEQ: "tag:yaml.org,2002:seq",
      STR: "tag:yaml.org,2002:str"
    };
    function findLineStarts(src) {
      const ls = [0];
      let offset = src.indexOf("\n");
      while (offset !== -1) {
        offset += 1;
        ls.push(offset);
        offset = src.indexOf("\n", offset);
      }
      return ls;
    }
    function getSrcInfo(cst) {
      let lineStarts, src;
      if (typeof cst === "string") {
        lineStarts = findLineStarts(cst);
        src = cst;
      } else {
        if (Array.isArray(cst)) cst = cst[0];
        if (cst && cst.context) {
          if (!cst.lineStarts) cst.lineStarts = findLineStarts(cst.context.src);
          lineStarts = cst.lineStarts;
          src = cst.context.src;
        }
      }
      return {
        lineStarts,
        src
      };
    }
    function getLinePos(offset, cst) {
      if (typeof offset !== "number" || offset < 0) return null;
      const {
        lineStarts,
        src
      } = getSrcInfo(cst);
      if (!lineStarts || !src || offset > src.length) return null;
      for (let i = 0; i < lineStarts.length; ++i) {
        const start = lineStarts[i];
        if (offset < start) {
          return {
            line: i,
            col: offset - lineStarts[i - 1] + 1
          };
        }
        if (offset === start) return {
          line: i + 1,
          col: 1
        };
      }
      const line = lineStarts.length;
      return {
        line,
        col: offset - lineStarts[line - 1] + 1
      };
    }
    function getLine(line, cst) {
      const {
        lineStarts,
        src
      } = getSrcInfo(cst);
      if (!lineStarts || !(line >= 1) || line > lineStarts.length) return null;
      const start = lineStarts[line - 1];
      let end = lineStarts[line];
      while (end && end > start && src[end - 1] === "\n") --end;
      return src.slice(start, end);
    }
    function getPrettyContext({
      start,
      end
    }, cst, maxWidth = 80) {
      let src = getLine(start.line, cst);
      if (!src) return null;
      let {
        col
      } = start;
      if (src.length > maxWidth) {
        if (col <= maxWidth - 10) {
          src = src.substr(0, maxWidth - 1) + "\u2026";
        } else {
          const halfWidth = Math.round(maxWidth / 2);
          if (src.length > col + halfWidth) src = src.substr(0, col + halfWidth - 1) + "\u2026";
          col -= src.length - maxWidth;
          src = "\u2026" + src.substr(1 - maxWidth);
        }
      }
      let errLen = 1;
      let errEnd = "";
      if (end) {
        if (end.line === start.line && col + (end.col - start.col) <= maxWidth + 1) {
          errLen = end.col - start.col;
        } else {
          errLen = Math.min(src.length + 1, maxWidth) - col;
          errEnd = "\u2026";
        }
      }
      const offset = col > 1 ? " ".repeat(col - 1) : "";
      const err = "^".repeat(errLen);
      return `${src}
${offset}${err}${errEnd}`;
    }
    var Range = class _Range {
      static copy(orig) {
        return new _Range(orig.start, orig.end);
      }
      constructor(start, end) {
        this.start = start;
        this.end = end || start;
      }
      isEmpty() {
        return typeof this.start !== "number" || !this.end || this.end <= this.start;
      }
      /**
       * Set `origStart` and `origEnd` to point to the original source range for
       * this node, which may differ due to dropped CR characters.
       *
       * @param {number[]} cr - Positions of dropped CR characters
       * @param {number} offset - Starting index of `cr` from the last call
       * @returns {number} - The next offset, matching the one found for `origStart`
       */
      setOrigRange(cr, offset) {
        const {
          start,
          end
        } = this;
        if (cr.length === 0 || end <= cr[0]) {
          this.origStart = start;
          this.origEnd = end;
          return offset;
        }
        let i = offset;
        while (i < cr.length) {
          if (cr[i] > start) break;
          else ++i;
        }
        this.origStart = start + i;
        const nextOffset = i;
        while (i < cr.length) {
          if (cr[i] >= end) break;
          else ++i;
        }
        this.origEnd = end + i;
        return nextOffset;
      }
    };
    var Node = class _Node {
      static addStringTerminator(src, offset, str) {
        if (str[str.length - 1] === "\n") return str;
        const next = _Node.endOfWhiteSpace(src, offset);
        return next >= src.length || src[next] === "\n" ? str + "\n" : str;
      }
      // ^(---|...)
      static atDocumentBoundary(src, offset, sep) {
        const ch0 = src[offset];
        if (!ch0) return true;
        const prev = src[offset - 1];
        if (prev && prev !== "\n") return false;
        if (sep) {
          if (ch0 !== sep) return false;
        } else {
          if (ch0 !== Char.DIRECTIVES_END && ch0 !== Char.DOCUMENT_END) return false;
        }
        const ch1 = src[offset + 1];
        const ch2 = src[offset + 2];
        if (ch1 !== ch0 || ch2 !== ch0) return false;
        const ch3 = src[offset + 3];
        return !ch3 || ch3 === "\n" || ch3 === "	" || ch3 === " ";
      }
      static endOfIdentifier(src, offset) {
        let ch = src[offset];
        const isVerbatim = ch === "<";
        const notOk = isVerbatim ? ["\n", "	", " ", ">"] : ["\n", "	", " ", "[", "]", "{", "}", ","];
        while (ch && notOk.indexOf(ch) === -1) ch = src[offset += 1];
        if (isVerbatim && ch === ">") offset += 1;
        return offset;
      }
      static endOfIndent(src, offset) {
        let ch = src[offset];
        while (ch === " ") ch = src[offset += 1];
        return offset;
      }
      static endOfLine(src, offset) {
        let ch = src[offset];
        while (ch && ch !== "\n") ch = src[offset += 1];
        return offset;
      }
      static endOfWhiteSpace(src, offset) {
        let ch = src[offset];
        while (ch === "	" || ch === " ") ch = src[offset += 1];
        return offset;
      }
      static startOfLine(src, offset) {
        let ch = src[offset - 1];
        if (ch === "\n") return offset;
        while (ch && ch !== "\n") ch = src[offset -= 1];
        return offset + 1;
      }
      /**
       * End of indentation, or null if the line's indent level is not more
       * than `indent`
       *
       * @param {string} src
       * @param {number} indent
       * @param {number} lineStart
       * @returns {?number}
       */
      static endOfBlockIndent(src, indent, lineStart) {
        const inEnd = _Node.endOfIndent(src, lineStart);
        if (inEnd > lineStart + indent) {
          return inEnd;
        } else {
          const wsEnd = _Node.endOfWhiteSpace(src, inEnd);
          const ch = src[wsEnd];
          if (!ch || ch === "\n") return wsEnd;
        }
        return null;
      }
      static atBlank(src, offset, endAsBlank) {
        const ch = src[offset];
        return ch === "\n" || ch === "	" || ch === " " || endAsBlank && !ch;
      }
      static nextNodeIsIndented(ch, indentDiff, indicatorAsIndent) {
        if (!ch || indentDiff < 0) return false;
        if (indentDiff > 0) return true;
        return indicatorAsIndent && ch === "-";
      }
      // should be at line or string end, or at next non-whitespace char
      static normalizeOffset(src, offset) {
        const ch = src[offset];
        return !ch ? offset : ch !== "\n" && src[offset - 1] === "\n" ? offset - 1 : _Node.endOfWhiteSpace(src, offset);
      }
      // fold single newline into space, multiple newlines to N - 1 newlines
      // presumes src[offset] === '\n'
      static foldNewline(src, offset, indent) {
        let inCount = 0;
        let error = false;
        let fold = "";
        let ch = src[offset + 1];
        while (ch === " " || ch === "	" || ch === "\n") {
          switch (ch) {
            case "\n":
              inCount = 0;
              offset += 1;
              fold += "\n";
              break;
            case "	":
              if (inCount <= indent) error = true;
              offset = _Node.endOfWhiteSpace(src, offset + 2) - 1;
              break;
            case " ":
              inCount += 1;
              offset += 1;
              break;
          }
          ch = src[offset + 1];
        }
        if (!fold) fold = " ";
        if (ch && inCount <= indent) error = true;
        return {
          fold,
          offset,
          error
        };
      }
      constructor(type, props, context) {
        Object.defineProperty(this, "context", {
          value: context || null,
          writable: true
        });
        this.error = null;
        this.range = null;
        this.valueRange = null;
        this.props = props || [];
        this.type = type;
        this.value = null;
      }
      getPropValue(idx, key, skipKey) {
        if (!this.context) return null;
        const {
          src
        } = this.context;
        const prop = this.props[idx];
        return prop && src[prop.start] === key ? src.slice(prop.start + (skipKey ? 1 : 0), prop.end) : null;
      }
      get anchor() {
        for (let i = 0; i < this.props.length; ++i) {
          const anchor = this.getPropValue(i, Char.ANCHOR, true);
          if (anchor != null) return anchor;
        }
        return null;
      }
      get comment() {
        const comments = [];
        for (let i = 0; i < this.props.length; ++i) {
          const comment = this.getPropValue(i, Char.COMMENT, true);
          if (comment != null) comments.push(comment);
        }
        return comments.length > 0 ? comments.join("\n") : null;
      }
      commentHasRequiredWhitespace(start) {
        const {
          src
        } = this.context;
        if (this.header && start === this.header.end) return false;
        if (!this.valueRange) return false;
        const {
          end
        } = this.valueRange;
        return start !== end || _Node.atBlank(src, end - 1);
      }
      get hasComment() {
        if (this.context) {
          const {
            src
          } = this.context;
          for (let i = 0; i < this.props.length; ++i) {
            if (src[this.props[i].start] === Char.COMMENT) return true;
          }
        }
        return false;
      }
      get hasProps() {
        if (this.context) {
          const {
            src
          } = this.context;
          for (let i = 0; i < this.props.length; ++i) {
            if (src[this.props[i].start] !== Char.COMMENT) return true;
          }
        }
        return false;
      }
      get includesTrailingLines() {
        return false;
      }
      get jsonLike() {
        const jsonLikeTypes = [Type.FLOW_MAP, Type.FLOW_SEQ, Type.QUOTE_DOUBLE, Type.QUOTE_SINGLE];
        return jsonLikeTypes.indexOf(this.type) !== -1;
      }
      get rangeAsLinePos() {
        if (!this.range || !this.context) return void 0;
        const start = getLinePos(this.range.start, this.context.root);
        if (!start) return void 0;
        const end = getLinePos(this.range.end, this.context.root);
        return {
          start,
          end
        };
      }
      get rawValue() {
        if (!this.valueRange || !this.context) return null;
        const {
          start,
          end
        } = this.valueRange;
        return this.context.src.slice(start, end);
      }
      get tag() {
        for (let i = 0; i < this.props.length; ++i) {
          const tag = this.getPropValue(i, Char.TAG, false);
          if (tag != null) {
            if (tag[1] === "<") {
              return {
                verbatim: tag.slice(2, -1)
              };
            } else {
              const [_, handle, suffix] = tag.match(/^(.*!)([^!]*)$/);
              return {
                handle,
                suffix
              };
            }
          }
        }
        return null;
      }
      get valueRangeContainsNewline() {
        if (!this.valueRange || !this.context) return false;
        const {
          start,
          end
        } = this.valueRange;
        const {
          src
        } = this.context;
        for (let i = start; i < end; ++i) {
          if (src[i] === "\n") return true;
        }
        return false;
      }
      parseComment(start) {
        const {
          src
        } = this.context;
        if (src[start] === Char.COMMENT) {
          const end = _Node.endOfLine(src, start + 1);
          const commentRange = new Range(start, end);
          this.props.push(commentRange);
          return end;
        }
        return start;
      }
      /**
       * Populates the `origStart` and `origEnd` values of all ranges for this
       * node. Extended by child classes to handle descendant nodes.
       *
       * @param {number[]} cr - Positions of dropped CR characters
       * @param {number} offset - Starting index of `cr` from the last call
       * @returns {number} - The next offset, matching the one found for `origStart`
       */
      setOrigRanges(cr, offset) {
        if (this.range) offset = this.range.setOrigRange(cr, offset);
        if (this.valueRange) this.valueRange.setOrigRange(cr, offset);
        this.props.forEach((prop) => prop.setOrigRange(cr, offset));
        return offset;
      }
      toString() {
        const {
          context: {
            src
          },
          range,
          value
        } = this;
        if (value != null) return value;
        const str = src.slice(range.start, range.end);
        return _Node.addStringTerminator(src, range.end, str);
      }
    };
    var YAMLError = class extends Error {
      constructor(name, source, message) {
        if (!message || !(source instanceof Node)) throw new Error(`Invalid arguments for new ${name}`);
        super();
        this.name = name;
        this.message = message;
        this.source = source;
      }
      makePretty() {
        if (!this.source) return;
        this.nodeType = this.source.type;
        const cst = this.source.context && this.source.context.root;
        if (typeof this.offset === "number") {
          this.range = new Range(this.offset, this.offset + 1);
          const start = cst && getLinePos(this.offset, cst);
          if (start) {
            const end = {
              line: start.line,
              col: start.col + 1
            };
            this.linePos = {
              start,
              end
            };
          }
          delete this.offset;
        } else {
          this.range = this.source.range;
          this.linePos = this.source.rangeAsLinePos;
        }
        if (this.linePos) {
          const {
            line,
            col
          } = this.linePos.start;
          this.message += ` at line ${line}, column ${col}`;
          const ctx = cst && getPrettyContext(this.linePos, cst);
          if (ctx) this.message += `:

${ctx}
`;
        }
        delete this.source;
      }
    };
    var YAMLReferenceError = class extends YAMLError {
      constructor(source, message) {
        super("YAMLReferenceError", source, message);
      }
    };
    var YAMLSemanticError = class extends YAMLError {
      constructor(source, message) {
        super("YAMLSemanticError", source, message);
      }
    };
    var YAMLSyntaxError = class extends YAMLError {
      constructor(source, message) {
        super("YAMLSyntaxError", source, message);
      }
    };
    var YAMLWarning = class extends YAMLError {
      constructor(source, message) {
        super("YAMLWarning", source, message);
      }
    };
    function _defineProperty(obj, key, value) {
      if (key in obj) {
        Object.defineProperty(obj, key, {
          value,
          enumerable: true,
          configurable: true,
          writable: true
        });
      } else {
        obj[key] = value;
      }
      return obj;
    }
    var PlainValue = class _PlainValue extends Node {
      static endOfLine(src, start, inFlow) {
        let ch = src[start];
        let offset = start;
        while (ch && ch !== "\n") {
          if (inFlow && (ch === "[" || ch === "]" || ch === "{" || ch === "}" || ch === ",")) break;
          const next = src[offset + 1];
          if (ch === ":" && (!next || next === "\n" || next === "	" || next === " " || inFlow && next === ",")) break;
          if ((ch === " " || ch === "	") && next === "#") break;
          offset += 1;
          ch = next;
        }
        return offset;
      }
      get strValue() {
        if (!this.valueRange || !this.context) return null;
        let {
          start,
          end
        } = this.valueRange;
        const {
          src
        } = this.context;
        let ch = src[end - 1];
        while (start < end && (ch === "\n" || ch === "	" || ch === " ")) ch = src[--end - 1];
        let str = "";
        for (let i = start; i < end; ++i) {
          const ch2 = src[i];
          if (ch2 === "\n") {
            const {
              fold,
              offset
            } = Node.foldNewline(src, i, -1);
            str += fold;
            i = offset;
          } else if (ch2 === " " || ch2 === "	") {
            const wsStart = i;
            let next = src[i + 1];
            while (i < end && (next === " " || next === "	")) {
              i += 1;
              next = src[i + 1];
            }
            if (next !== "\n") str += i > wsStart ? src.slice(wsStart, i + 1) : ch2;
          } else {
            str += ch2;
          }
        }
        const ch0 = src[start];
        switch (ch0) {
          case "	": {
            const msg = "Plain value cannot start with a tab character";
            const errors = [new YAMLSemanticError(this, msg)];
            return {
              errors,
              str
            };
          }
          case "@":
          case "`": {
            const msg = `Plain value cannot start with reserved character ${ch0}`;
            const errors = [new YAMLSemanticError(this, msg)];
            return {
              errors,
              str
            };
          }
          default:
            return str;
        }
      }
      parseBlockValue(start) {
        const {
          indent,
          inFlow,
          src
        } = this.context;
        let offset = start;
        let valueEnd = start;
        for (let ch = src[offset]; ch === "\n"; ch = src[offset]) {
          if (Node.atDocumentBoundary(src, offset + 1)) break;
          const end = Node.endOfBlockIndent(src, indent, offset + 1);
          if (end === null || src[end] === "#") break;
          if (src[end] === "\n") {
            offset = end;
          } else {
            valueEnd = _PlainValue.endOfLine(src, end, inFlow);
            offset = valueEnd;
          }
        }
        if (this.valueRange.isEmpty()) this.valueRange.start = start;
        this.valueRange.end = valueEnd;
        return valueEnd;
      }
      /**
       * Parses a plain value from the source
       *
       * Accepted forms are:
       * ```
       * #comment
       *
       * first line
       *
       * first line #comment
       *
       * first line
       * block
       * lines
       *
       * #comment
       * block
       * lines
       * ```
       * where block lines are empty or have an indent level greater than `indent`.
       *
       * @param {ParseContext} context
       * @param {number} start - Index of first character
       * @returns {number} - Index of the character after this scalar, may be `\n`
       */
      parse(context, start) {
        this.context = context;
        const {
          inFlow,
          src
        } = context;
        let offset = start;
        const ch = src[offset];
        if (ch && ch !== "#" && ch !== "\n") {
          offset = _PlainValue.endOfLine(src, start, inFlow);
        }
        this.valueRange = new Range(start, offset);
        offset = Node.endOfWhiteSpace(src, offset);
        offset = this.parseComment(offset);
        if (!this.hasComment || this.valueRange.isEmpty()) {
          offset = this.parseBlockValue(offset);
        }
        return offset;
      }
    };
    exports2.Char = Char;
    exports2.Node = Node;
    exports2.PlainValue = PlainValue;
    exports2.Range = Range;
    exports2.Type = Type;
    exports2.YAMLError = YAMLError;
    exports2.YAMLReferenceError = YAMLReferenceError;
    exports2.YAMLSemanticError = YAMLSemanticError;
    exports2.YAMLSyntaxError = YAMLSyntaxError;
    exports2.YAMLWarning = YAMLWarning;
    exports2._defineProperty = _defineProperty;
    exports2.defaultTagPrefix = defaultTagPrefix;
    exports2.defaultTags = defaultTags;
  }
});

// node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/parse-cst.js
var require_parse_cst = __commonJS({
  "node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/parse-cst.js"(exports2) {
    "use strict";
    var PlainValue = require_PlainValue_ec8e588e();
    var BlankLine = class extends PlainValue.Node {
      constructor() {
        super(PlainValue.Type.BLANK_LINE);
      }
      /* istanbul ignore next */
      get includesTrailingLines() {
        return true;
      }
      /**
       * Parses a blank line from the source
       *
       * @param {ParseContext} context
       * @param {number} start - Index of first \n character
       * @returns {number} - Index of the character after this
       */
      parse(context, start) {
        this.context = context;
        this.range = new PlainValue.Range(start, start + 1);
        return start + 1;
      }
    };
    var CollectionItem = class extends PlainValue.Node {
      constructor(type, props) {
        super(type, props);
        this.node = null;
      }
      get includesTrailingLines() {
        return !!this.node && this.node.includesTrailingLines;
      }
      /**
       * @param {ParseContext} context
       * @param {number} start - Index of first character
       * @returns {number} - Index of the character after this
       */
      parse(context, start) {
        this.context = context;
        const {
          parseNode,
          src
        } = context;
        let {
          atLineStart,
          lineStart
        } = context;
        if (!atLineStart && this.type === PlainValue.Type.SEQ_ITEM) this.error = new PlainValue.YAMLSemanticError(this, "Sequence items must not have preceding content on the same line");
        const indent = atLineStart ? start - lineStart : context.indent;
        let offset = PlainValue.Node.endOfWhiteSpace(src, start + 1);
        let ch = src[offset];
        const inlineComment = ch === "#";
        const comments = [];
        let blankLine = null;
        while (ch === "\n" || ch === "#") {
          if (ch === "#") {
            const end2 = PlainValue.Node.endOfLine(src, offset + 1);
            comments.push(new PlainValue.Range(offset, end2));
            offset = end2;
          } else {
            atLineStart = true;
            lineStart = offset + 1;
            const wsEnd = PlainValue.Node.endOfWhiteSpace(src, lineStart);
            if (src[wsEnd] === "\n" && comments.length === 0) {
              blankLine = new BlankLine();
              lineStart = blankLine.parse({
                src
              }, lineStart);
            }
            offset = PlainValue.Node.endOfIndent(src, lineStart);
          }
          ch = src[offset];
        }
        if (PlainValue.Node.nextNodeIsIndented(ch, offset - (lineStart + indent), this.type !== PlainValue.Type.SEQ_ITEM)) {
          this.node = parseNode({
            atLineStart,
            inCollection: false,
            indent,
            lineStart,
            parent: this
          }, offset);
        } else if (ch && lineStart > start + 1) {
          offset = lineStart - 1;
        }
        if (this.node) {
          if (blankLine) {
            const items = context.parent.items || context.parent.contents;
            if (items) items.push(blankLine);
          }
          if (comments.length) Array.prototype.push.apply(this.props, comments);
          offset = this.node.range.end;
        } else {
          if (inlineComment) {
            const c = comments[0];
            this.props.push(c);
            offset = c.end;
          } else {
            offset = PlainValue.Node.endOfLine(src, start + 1);
          }
        }
        const end = this.node ? this.node.valueRange.end : offset;
        this.valueRange = new PlainValue.Range(start, end);
        return offset;
      }
      setOrigRanges(cr, offset) {
        offset = super.setOrigRanges(cr, offset);
        return this.node ? this.node.setOrigRanges(cr, offset) : offset;
      }
      toString() {
        const {
          context: {
            src
          },
          node,
          range,
          value
        } = this;
        if (value != null) return value;
        const str = node ? src.slice(range.start, node.range.start) + String(node) : src.slice(range.start, range.end);
        return PlainValue.Node.addStringTerminator(src, range.end, str);
      }
    };
    var Comment = class extends PlainValue.Node {
      constructor() {
        super(PlainValue.Type.COMMENT);
      }
      /**
       * Parses a comment line from the source
       *
       * @param {ParseContext} context
       * @param {number} start - Index of first character
       * @returns {number} - Index of the character after this scalar
       */
      parse(context, start) {
        this.context = context;
        const offset = this.parseComment(start);
        this.range = new PlainValue.Range(start, offset);
        return offset;
      }
    };
    function grabCollectionEndComments(node) {
      let cnode = node;
      while (cnode instanceof CollectionItem) cnode = cnode.node;
      if (!(cnode instanceof Collection)) return null;
      const len = cnode.items.length;
      let ci = -1;
      for (let i = len - 1; i >= 0; --i) {
        const n = cnode.items[i];
        if (n.type === PlainValue.Type.COMMENT) {
          const {
            indent,
            lineStart
          } = n.context;
          if (indent > 0 && n.range.start >= lineStart + indent) break;
          ci = i;
        } else if (n.type === PlainValue.Type.BLANK_LINE) ci = i;
        else break;
      }
      if (ci === -1) return null;
      const ca = cnode.items.splice(ci, len - ci);
      const prevEnd = ca[0].range.start;
      while (true) {
        cnode.range.end = prevEnd;
        if (cnode.valueRange && cnode.valueRange.end > prevEnd) cnode.valueRange.end = prevEnd;
        if (cnode === node) break;
        cnode = cnode.context.parent;
      }
      return ca;
    }
    var Collection = class _Collection extends PlainValue.Node {
      static nextContentHasIndent(src, offset, indent) {
        const lineStart = PlainValue.Node.endOfLine(src, offset) + 1;
        offset = PlainValue.Node.endOfWhiteSpace(src, lineStart);
        const ch = src[offset];
        if (!ch) return false;
        if (offset >= lineStart + indent) return true;
        if (ch !== "#" && ch !== "\n") return false;
        return _Collection.nextContentHasIndent(src, offset, indent);
      }
      constructor(firstItem) {
        super(firstItem.type === PlainValue.Type.SEQ_ITEM ? PlainValue.Type.SEQ : PlainValue.Type.MAP);
        for (let i = firstItem.props.length - 1; i >= 0; --i) {
          if (firstItem.props[i].start < firstItem.context.lineStart) {
            this.props = firstItem.props.slice(0, i + 1);
            firstItem.props = firstItem.props.slice(i + 1);
            const itemRange = firstItem.props[0] || firstItem.valueRange;
            firstItem.range.start = itemRange.start;
            break;
          }
        }
        this.items = [firstItem];
        const ec = grabCollectionEndComments(firstItem);
        if (ec) Array.prototype.push.apply(this.items, ec);
      }
      get includesTrailingLines() {
        return this.items.length > 0;
      }
      /**
       * @param {ParseContext} context
       * @param {number} start - Index of first character
       * @returns {number} - Index of the character after this
       */
      parse(context, start) {
        this.context = context;
        const {
          parseNode,
          src
        } = context;
        let lineStart = PlainValue.Node.startOfLine(src, start);
        const firstItem = this.items[0];
        firstItem.context.parent = this;
        this.valueRange = PlainValue.Range.copy(firstItem.valueRange);
        const indent = firstItem.range.start - firstItem.context.lineStart;
        let offset = start;
        offset = PlainValue.Node.normalizeOffset(src, offset);
        let ch = src[offset];
        let atLineStart = PlainValue.Node.endOfWhiteSpace(src, lineStart) === offset;
        let prevIncludesTrailingLines = false;
        while (ch) {
          while (ch === "\n" || ch === "#") {
            if (atLineStart && ch === "\n" && !prevIncludesTrailingLines) {
              const blankLine = new BlankLine();
              offset = blankLine.parse({
                src
              }, offset);
              this.valueRange.end = offset;
              if (offset >= src.length) {
                ch = null;
                break;
              }
              this.items.push(blankLine);
              offset -= 1;
            } else if (ch === "#") {
              if (offset < lineStart + indent && !_Collection.nextContentHasIndent(src, offset, indent)) {
                return offset;
              }
              const comment = new Comment();
              offset = comment.parse({
                indent,
                lineStart,
                src
              }, offset);
              this.items.push(comment);
              this.valueRange.end = offset;
              if (offset >= src.length) {
                ch = null;
                break;
              }
            }
            lineStart = offset + 1;
            offset = PlainValue.Node.endOfIndent(src, lineStart);
            if (PlainValue.Node.atBlank(src, offset)) {
              const wsEnd = PlainValue.Node.endOfWhiteSpace(src, offset);
              const next = src[wsEnd];
              if (!next || next === "\n" || next === "#") {
                offset = wsEnd;
              }
            }
            ch = src[offset];
            atLineStart = true;
          }
          if (!ch) {
            break;
          }
          if (offset !== lineStart + indent && (atLineStart || ch !== ":")) {
            if (offset < lineStart + indent) {
              if (lineStart > start) offset = lineStart;
              break;
            } else if (!this.error) {
              const msg = "All collection items must start at the same column";
              this.error = new PlainValue.YAMLSyntaxError(this, msg);
            }
          }
          if (firstItem.type === PlainValue.Type.SEQ_ITEM) {
            if (ch !== "-") {
              if (lineStart > start) offset = lineStart;
              break;
            }
          } else if (ch === "-" && !this.error) {
            const next = src[offset + 1];
            if (!next || next === "\n" || next === "	" || next === " ") {
              const msg = "A collection cannot be both a mapping and a sequence";
              this.error = new PlainValue.YAMLSyntaxError(this, msg);
            }
          }
          const node = parseNode({
            atLineStart,
            inCollection: true,
            indent,
            lineStart,
            parent: this
          }, offset);
          if (!node) return offset;
          this.items.push(node);
          this.valueRange.end = node.valueRange.end;
          offset = PlainValue.Node.normalizeOffset(src, node.range.end);
          ch = src[offset];
          atLineStart = false;
          prevIncludesTrailingLines = node.includesTrailingLines;
          if (ch) {
            let ls = offset - 1;
            let prev = src[ls];
            while (prev === " " || prev === "	") prev = src[--ls];
            if (prev === "\n") {
              lineStart = ls + 1;
              atLineStart = true;
            }
          }
          const ec = grabCollectionEndComments(node);
          if (ec) Array.prototype.push.apply(this.items, ec);
        }
        return offset;
      }
      setOrigRanges(cr, offset) {
        offset = super.setOrigRanges(cr, offset);
        this.items.forEach((node) => {
          offset = node.setOrigRanges(cr, offset);
        });
        return offset;
      }
      toString() {
        const {
          context: {
            src
          },
          items,
          range,
          value
        } = this;
        if (value != null) return value;
        let str = src.slice(range.start, items[0].range.start) + String(items[0]);
        for (let i = 1; i < items.length; ++i) {
          const item = items[i];
          const {
            atLineStart,
            indent
          } = item.context;
          if (atLineStart) for (let i2 = 0; i2 < indent; ++i2) str += " ";
          str += String(item);
        }
        return PlainValue.Node.addStringTerminator(src, range.end, str);
      }
    };
    var Directive = class extends PlainValue.Node {
      constructor() {
        super(PlainValue.Type.DIRECTIVE);
        this.name = null;
      }
      get parameters() {
        const raw = this.rawValue;
        return raw ? raw.trim().split(/[ \t]+/) : [];
      }
      parseName(start) {
        const {
          src
        } = this.context;
        let offset = start;
        let ch = src[offset];
        while (ch && ch !== "\n" && ch !== "	" && ch !== " ") ch = src[offset += 1];
        this.name = src.slice(start, offset);
        return offset;
      }
      parseParameters(start) {
        const {
          src
        } = this.context;
        let offset = start;
        let ch = src[offset];
        while (ch && ch !== "\n" && ch !== "#") ch = src[offset += 1];
        this.valueRange = new PlainValue.Range(start, offset);
        return offset;
      }
      parse(context, start) {
        this.context = context;
        let offset = this.parseName(start + 1);
        offset = this.parseParameters(offset);
        offset = this.parseComment(offset);
        this.range = new PlainValue.Range(start, offset);
        return offset;
      }
    };
    var Document = class _Document extends PlainValue.Node {
      static startCommentOrEndBlankLine(src, start) {
        const offset = PlainValue.Node.endOfWhiteSpace(src, start);
        const ch = src[offset];
        return ch === "#" || ch === "\n" ? offset : start;
      }
      constructor() {
        super(PlainValue.Type.DOCUMENT);
        this.directives = null;
        this.contents = null;
        this.directivesEndMarker = null;
        this.documentEndMarker = null;
      }
      parseDirectives(start) {
        const {
          src
        } = this.context;
        this.directives = [];
        let atLineStart = true;
        let hasDirectives = false;
        let offset = start;
        while (!PlainValue.Node.atDocumentBoundary(src, offset, PlainValue.Char.DIRECTIVES_END)) {
          offset = _Document.startCommentOrEndBlankLine(src, offset);
          switch (src[offset]) {
            case "\n":
              if (atLineStart) {
                const blankLine = new BlankLine();
                offset = blankLine.parse({
                  src
                }, offset);
                if (offset < src.length) {
                  this.directives.push(blankLine);
                }
              } else {
                offset += 1;
                atLineStart = true;
              }
              break;
            case "#":
              {
                const comment = new Comment();
                offset = comment.parse({
                  src
                }, offset);
                this.directives.push(comment);
                atLineStart = false;
              }
              break;
            case "%":
              {
                const directive = new Directive();
                offset = directive.parse({
                  parent: this,
                  src
                }, offset);
                this.directives.push(directive);
                hasDirectives = true;
                atLineStart = false;
              }
              break;
            default:
              if (hasDirectives) {
                this.error = new PlainValue.YAMLSemanticError(this, "Missing directives-end indicator line");
              } else if (this.directives.length > 0) {
                this.contents = this.directives;
                this.directives = [];
              }
              return offset;
          }
        }
        if (src[offset]) {
          this.directivesEndMarker = new PlainValue.Range(offset, offset + 3);
          return offset + 3;
        }
        if (hasDirectives) {
          this.error = new PlainValue.YAMLSemanticError(this, "Missing directives-end indicator line");
        } else if (this.directives.length > 0) {
          this.contents = this.directives;
          this.directives = [];
        }
        return offset;
      }
      parseContents(start) {
        const {
          parseNode,
          src
        } = this.context;
        if (!this.contents) this.contents = [];
        let lineStart = start;
        while (src[lineStart - 1] === "-") lineStart -= 1;
        let offset = PlainValue.Node.endOfWhiteSpace(src, start);
        let atLineStart = lineStart === start;
        this.valueRange = new PlainValue.Range(offset);
        while (!PlainValue.Node.atDocumentBoundary(src, offset, PlainValue.Char.DOCUMENT_END)) {
          switch (src[offset]) {
            case "\n":
              if (atLineStart) {
                const blankLine = new BlankLine();
                offset = blankLine.parse({
                  src
                }, offset);
                if (offset < src.length) {
                  this.contents.push(blankLine);
                }
              } else {
                offset += 1;
                atLineStart = true;
              }
              lineStart = offset;
              break;
            case "#":
              {
                const comment = new Comment();
                offset = comment.parse({
                  src
                }, offset);
                this.contents.push(comment);
                atLineStart = false;
              }
              break;
            default: {
              const iEnd = PlainValue.Node.endOfIndent(src, offset);
              const context = {
                atLineStart,
                indent: -1,
                inFlow: false,
                inCollection: false,
                lineStart,
                parent: this
              };
              const node = parseNode(context, iEnd);
              if (!node) return this.valueRange.end = iEnd;
              this.contents.push(node);
              offset = node.range.end;
              atLineStart = false;
              const ec = grabCollectionEndComments(node);
              if (ec) Array.prototype.push.apply(this.contents, ec);
            }
          }
          offset = _Document.startCommentOrEndBlankLine(src, offset);
        }
        this.valueRange.end = offset;
        if (src[offset]) {
          this.documentEndMarker = new PlainValue.Range(offset, offset + 3);
          offset += 3;
          if (src[offset]) {
            offset = PlainValue.Node.endOfWhiteSpace(src, offset);
            if (src[offset] === "#") {
              const comment = new Comment();
              offset = comment.parse({
                src
              }, offset);
              this.contents.push(comment);
            }
            switch (src[offset]) {
              case "\n":
                offset += 1;
                break;
              case void 0:
                break;
              default:
                this.error = new PlainValue.YAMLSyntaxError(this, "Document end marker line cannot have a non-comment suffix");
            }
          }
        }
        return offset;
      }
      /**
       * @param {ParseContext} context
       * @param {number} start - Index of first character
       * @returns {number} - Index of the character after this
       */
      parse(context, start) {
        context.root = this;
        this.context = context;
        const {
          src
        } = context;
        let offset = src.charCodeAt(start) === 65279 ? start + 1 : start;
        offset = this.parseDirectives(offset);
        offset = this.parseContents(offset);
        return offset;
      }
      setOrigRanges(cr, offset) {
        offset = super.setOrigRanges(cr, offset);
        this.directives.forEach((node) => {
          offset = node.setOrigRanges(cr, offset);
        });
        if (this.directivesEndMarker) offset = this.directivesEndMarker.setOrigRange(cr, offset);
        this.contents.forEach((node) => {
          offset = node.setOrigRanges(cr, offset);
        });
        if (this.documentEndMarker) offset = this.documentEndMarker.setOrigRange(cr, offset);
        return offset;
      }
      toString() {
        const {
          contents,
          directives,
          value
        } = this;
        if (value != null) return value;
        let str = directives.join("");
        if (contents.length > 0) {
          if (directives.length > 0 || contents[0].type === PlainValue.Type.COMMENT) str += "---\n";
          str += contents.join("");
        }
        if (str[str.length - 1] !== "\n") str += "\n";
        return str;
      }
    };
    var Alias = class extends PlainValue.Node {
      /**
       * Parses an *alias from the source
       *
       * @param {ParseContext} context
       * @param {number} start - Index of first character
       * @returns {number} - Index of the character after this scalar
       */
      parse(context, start) {
        this.context = context;
        const {
          src
        } = context;
        let offset = PlainValue.Node.endOfIdentifier(src, start + 1);
        this.valueRange = new PlainValue.Range(start + 1, offset);
        offset = PlainValue.Node.endOfWhiteSpace(src, offset);
        offset = this.parseComment(offset);
        return offset;
      }
    };
    var Chomp = {
      CLIP: "CLIP",
      KEEP: "KEEP",
      STRIP: "STRIP"
    };
    var BlockValue = class extends PlainValue.Node {
      constructor(type, props) {
        super(type, props);
        this.blockIndent = null;
        this.chomping = Chomp.CLIP;
        this.header = null;
      }
      get includesTrailingLines() {
        return this.chomping === Chomp.KEEP;
      }
      get strValue() {
        if (!this.valueRange || !this.context) return null;
        let {
          start,
          end
        } = this.valueRange;
        const {
          indent,
          src
        } = this.context;
        if (this.valueRange.isEmpty()) return "";
        let lastNewLine = null;
        let ch = src[end - 1];
        while (ch === "\n" || ch === "	" || ch === " ") {
          end -= 1;
          if (end <= start) {
            if (this.chomping === Chomp.KEEP) break;
            else return "";
          }
          if (ch === "\n") lastNewLine = end;
          ch = src[end - 1];
        }
        let keepStart = end + 1;
        if (lastNewLine) {
          if (this.chomping === Chomp.KEEP) {
            keepStart = lastNewLine;
            end = this.valueRange.end;
          } else {
            end = lastNewLine;
          }
        }
        const bi = indent + this.blockIndent;
        const folded = this.type === PlainValue.Type.BLOCK_FOLDED;
        let atStart = true;
        let str = "";
        let sep = "";
        let prevMoreIndented = false;
        for (let i = start; i < end; ++i) {
          for (let j = 0; j < bi; ++j) {
            if (src[i] !== " ") break;
            i += 1;
          }
          const ch2 = src[i];
          if (ch2 === "\n") {
            if (sep === "\n") str += "\n";
            else sep = "\n";
          } else {
            const lineEnd = PlainValue.Node.endOfLine(src, i);
            const line = src.slice(i, lineEnd);
            i = lineEnd;
            if (folded && (ch2 === " " || ch2 === "	") && i < keepStart) {
              if (sep === " ") sep = "\n";
              else if (!prevMoreIndented && !atStart && sep === "\n") sep = "\n\n";
              str += sep + line;
              sep = lineEnd < end && src[lineEnd] || "";
              prevMoreIndented = true;
            } else {
              str += sep + line;
              sep = folded && i < keepStart ? " " : "\n";
              prevMoreIndented = false;
            }
            if (atStart && line !== "") atStart = false;
          }
        }
        return this.chomping === Chomp.STRIP ? str : str + "\n";
      }
      parseBlockHeader(start) {
        const {
          src
        } = this.context;
        let offset = start + 1;
        let bi = "";
        while (true) {
          const ch = src[offset];
          switch (ch) {
            case "-":
              this.chomping = Chomp.STRIP;
              break;
            case "+":
              this.chomping = Chomp.KEEP;
              break;
            case "0":
            case "1":
            case "2":
            case "3":
            case "4":
            case "5":
            case "6":
            case "7":
            case "8":
            case "9":
              bi += ch;
              break;
            default:
              this.blockIndent = Number(bi) || null;
              this.header = new PlainValue.Range(start, offset);
              return offset;
          }
          offset += 1;
        }
      }
      parseBlockValue(start) {
        const {
          indent,
          src
        } = this.context;
        const explicit = !!this.blockIndent;
        let offset = start;
        let valueEnd = start;
        let minBlockIndent = 1;
        for (let ch = src[offset]; ch === "\n"; ch = src[offset]) {
          offset += 1;
          if (PlainValue.Node.atDocumentBoundary(src, offset)) break;
          const end = PlainValue.Node.endOfBlockIndent(src, indent, offset);
          if (end === null) break;
          const ch2 = src[end];
          const lineIndent = end - (offset + indent);
          if (!this.blockIndent) {
            if (src[end] !== "\n") {
              if (lineIndent < minBlockIndent) {
                const msg = "Block scalars with more-indented leading empty lines must use an explicit indentation indicator";
                this.error = new PlainValue.YAMLSemanticError(this, msg);
              }
              this.blockIndent = lineIndent;
            } else if (lineIndent > minBlockIndent) {
              minBlockIndent = lineIndent;
            }
          } else if (ch2 && ch2 !== "\n" && lineIndent < this.blockIndent) {
            if (src[end] === "#") break;
            if (!this.error) {
              const src2 = explicit ? "explicit indentation indicator" : "first line";
              const msg = `Block scalars must not be less indented than their ${src2}`;
              this.error = new PlainValue.YAMLSemanticError(this, msg);
            }
          }
          if (src[end] === "\n") {
            offset = end;
          } else {
            offset = valueEnd = PlainValue.Node.endOfLine(src, end);
          }
        }
        if (this.chomping !== Chomp.KEEP) {
          offset = src[valueEnd] ? valueEnd + 1 : valueEnd;
        }
        this.valueRange = new PlainValue.Range(start + 1, offset);
        return offset;
      }
      /**
       * Parses a block value from the source
       *
       * Accepted forms are:
       * ```
       * BS
       * block
       * lines
       *
       * BS #comment
       * block
       * lines
       * ```
       * where the block style BS matches the regexp `[|>][-+1-9]*` and block lines
       * are empty or have an indent level greater than `indent`.
       *
       * @param {ParseContext} context
       * @param {number} start - Index of first character
       * @returns {number} - Index of the character after this block
       */
      parse(context, start) {
        this.context = context;
        const {
          src
        } = context;
        let offset = this.parseBlockHeader(start);
        offset = PlainValue.Node.endOfWhiteSpace(src, offset);
        offset = this.parseComment(offset);
        offset = this.parseBlockValue(offset);
        return offset;
      }
      setOrigRanges(cr, offset) {
        offset = super.setOrigRanges(cr, offset);
        return this.header ? this.header.setOrigRange(cr, offset) : offset;
      }
    };
    var FlowCollection = class extends PlainValue.Node {
      constructor(type, props) {
        super(type, props);
        this.items = null;
      }
      prevNodeIsJsonLike(idx = this.items.length) {
        const node = this.items[idx - 1];
        return !!node && (node.jsonLike || node.type === PlainValue.Type.COMMENT && this.prevNodeIsJsonLike(idx - 1));
      }
      /**
       * @param {ParseContext} context
       * @param {number} start - Index of first character
       * @returns {number} - Index of the character after this
       */
      parse(context, start) {
        this.context = context;
        const {
          parseNode,
          src
        } = context;
        let {
          indent,
          lineStart
        } = context;
        let char = src[start];
        this.items = [{
          char,
          offset: start
        }];
        let offset = PlainValue.Node.endOfWhiteSpace(src, start + 1);
        char = src[offset];
        while (char && char !== "]" && char !== "}") {
          switch (char) {
            case "\n":
              {
                lineStart = offset + 1;
                const wsEnd = PlainValue.Node.endOfWhiteSpace(src, lineStart);
                if (src[wsEnd] === "\n") {
                  const blankLine = new BlankLine();
                  lineStart = blankLine.parse({
                    src
                  }, lineStart);
                  this.items.push(blankLine);
                }
                offset = PlainValue.Node.endOfIndent(src, lineStart);
                if (offset <= lineStart + indent) {
                  char = src[offset];
                  if (offset < lineStart + indent || char !== "]" && char !== "}") {
                    const msg = "Insufficient indentation in flow collection";
                    this.error = new PlainValue.YAMLSemanticError(this, msg);
                  }
                }
              }
              break;
            case ",":
              {
                this.items.push({
                  char,
                  offset
                });
                offset += 1;
              }
              break;
            case "#":
              {
                const comment = new Comment();
                offset = comment.parse({
                  src
                }, offset);
                this.items.push(comment);
              }
              break;
            case "?":
            case ":": {
              const next = src[offset + 1];
              if (next === "\n" || next === "	" || next === " " || next === "," || // in-flow : after JSON-like key does not need to be followed by whitespace
              char === ":" && this.prevNodeIsJsonLike()) {
                this.items.push({
                  char,
                  offset
                });
                offset += 1;
                break;
              }
            }
            // fallthrough
            default: {
              const node = parseNode({
                atLineStart: false,
                inCollection: false,
                inFlow: true,
                indent: -1,
                lineStart,
                parent: this
              }, offset);
              if (!node) {
                this.valueRange = new PlainValue.Range(start, offset);
                return offset;
              }
              this.items.push(node);
              offset = PlainValue.Node.normalizeOffset(src, node.range.end);
            }
          }
          offset = PlainValue.Node.endOfWhiteSpace(src, offset);
          char = src[offset];
        }
        this.valueRange = new PlainValue.Range(start, offset + 1);
        if (char) {
          this.items.push({
            char,
            offset
          });
          offset = PlainValue.Node.endOfWhiteSpace(src, offset + 1);
          offset = this.parseComment(offset);
        }
        return offset;
      }
      setOrigRanges(cr, offset) {
        offset = super.setOrigRanges(cr, offset);
        this.items.forEach((node) => {
          if (node instanceof PlainValue.Node) {
            offset = node.setOrigRanges(cr, offset);
          } else if (cr.length === 0) {
            node.origOffset = node.offset;
          } else {
            let i = offset;
            while (i < cr.length) {
              if (cr[i] > node.offset) break;
              else ++i;
            }
            node.origOffset = node.offset + i;
            offset = i;
          }
        });
        return offset;
      }
      toString() {
        const {
          context: {
            src
          },
          items,
          range,
          value
        } = this;
        if (value != null) return value;
        const nodes = items.filter((item) => item instanceof PlainValue.Node);
        let str = "";
        let prevEnd = range.start;
        nodes.forEach((node) => {
          const prefix = src.slice(prevEnd, node.range.start);
          prevEnd = node.range.end;
          str += prefix + String(node);
          if (str[str.length - 1] === "\n" && src[prevEnd - 1] !== "\n" && src[prevEnd] === "\n") {
            prevEnd += 1;
          }
        });
        str += src.slice(prevEnd, range.end);
        return PlainValue.Node.addStringTerminator(src, range.end, str);
      }
    };
    var QuoteDouble = class _QuoteDouble extends PlainValue.Node {
      static endOfQuote(src, offset) {
        let ch = src[offset];
        while (ch && ch !== '"') {
          offset += ch === "\\" ? 2 : 1;
          ch = src[offset];
        }
        return offset + 1;
      }
      /**
       * @returns {string | { str: string, errors: YAMLSyntaxError[] }}
       */
      get strValue() {
        if (!this.valueRange || !this.context) return null;
        const errors = [];
        const {
          start,
          end
        } = this.valueRange;
        const {
          indent,
          src
        } = this.context;
        if (src[end - 1] !== '"') errors.push(new PlainValue.YAMLSyntaxError(this, 'Missing closing "quote'));
        let str = "";
        for (let i = start + 1; i < end - 1; ++i) {
          const ch = src[i];
          if (ch === "\n") {
            if (PlainValue.Node.atDocumentBoundary(src, i + 1)) errors.push(new PlainValue.YAMLSemanticError(this, "Document boundary indicators are not allowed within string values"));
            const {
              fold,
              offset,
              error
            } = PlainValue.Node.foldNewline(src, i, indent);
            str += fold;
            i = offset;
            if (error) errors.push(new PlainValue.YAMLSemanticError(this, "Multi-line double-quoted string needs to be sufficiently indented"));
          } else if (ch === "\\") {
            i += 1;
            switch (src[i]) {
              case "0":
                str += "\0";
                break;
              // null character
              case "a":
                str += "\x07";
                break;
              // bell character
              case "b":
                str += "\b";
                break;
              // backspace
              case "e":
                str += "\x1B";
                break;
              // escape character
              case "f":
                str += "\f";
                break;
              // form feed
              case "n":
                str += "\n";
                break;
              // line feed
              case "r":
                str += "\r";
                break;
              // carriage return
              case "t":
                str += "	";
                break;
              // horizontal tab
              case "v":
                str += "\v";
                break;
              // vertical tab
              case "N":
                str += "\x85";
                break;
              // Unicode next line
              case "_":
                str += "\xA0";
                break;
              // Unicode non-breaking space
              case "L":
                str += "\u2028";
                break;
              // Unicode line separator
              case "P":
                str += "\u2029";
                break;
              // Unicode paragraph separator
              case " ":
                str += " ";
                break;
              case '"':
                str += '"';
                break;
              case "/":
                str += "/";
                break;
              case "\\":
                str += "\\";
                break;
              case "	":
                str += "	";
                break;
              case "x":
                str += this.parseCharCode(i + 1, 2, errors);
                i += 2;
                break;
              case "u":
                str += this.parseCharCode(i + 1, 4, errors);
                i += 4;
                break;
              case "U":
                str += this.parseCharCode(i + 1, 8, errors);
                i += 8;
                break;
              case "\n":
                while (src[i + 1] === " " || src[i + 1] === "	") i += 1;
                break;
              default:
                errors.push(new PlainValue.YAMLSyntaxError(this, `Invalid escape sequence ${src.substr(i - 1, 2)}`));
                str += "\\" + src[i];
            }
          } else if (ch === " " || ch === "	") {
            const wsStart = i;
            let next = src[i + 1];
            while (next === " " || next === "	") {
              i += 1;
              next = src[i + 1];
            }
            if (next !== "\n") str += i > wsStart ? src.slice(wsStart, i + 1) : ch;
          } else {
            str += ch;
          }
        }
        return errors.length > 0 ? {
          errors,
          str
        } : str;
      }
      parseCharCode(offset, length, errors) {
        const {
          src
        } = this.context;
        const cc = src.substr(offset, length);
        const ok = cc.length === length && /^[0-9a-fA-F]+$/.test(cc);
        const code = ok ? parseInt(cc, 16) : NaN;
        if (isNaN(code)) {
          errors.push(new PlainValue.YAMLSyntaxError(this, `Invalid escape sequence ${src.substr(offset - 2, length + 2)}`));
          return src.substr(offset - 2, length + 2);
        }
        return String.fromCodePoint(code);
      }
      /**
       * Parses a "double quoted" value from the source
       *
       * @param {ParseContext} context
       * @param {number} start - Index of first character
       * @returns {number} - Index of the character after this scalar
       */
      parse(context, start) {
        this.context = context;
        const {
          src
        } = context;
        let offset = _QuoteDouble.endOfQuote(src, start + 1);
        this.valueRange = new PlainValue.Range(start, offset);
        offset = PlainValue.Node.endOfWhiteSpace(src, offset);
        offset = this.parseComment(offset);
        return offset;
      }
    };
    var QuoteSingle = class _QuoteSingle extends PlainValue.Node {
      static endOfQuote(src, offset) {
        let ch = src[offset];
        while (ch) {
          if (ch === "'") {
            if (src[offset + 1] !== "'") break;
            ch = src[offset += 2];
          } else {
            ch = src[offset += 1];
          }
        }
        return offset + 1;
      }
      /**
       * @returns {string | { str: string, errors: YAMLSyntaxError[] }}
       */
      get strValue() {
        if (!this.valueRange || !this.context) return null;
        const errors = [];
        const {
          start,
          end
        } = this.valueRange;
        const {
          indent,
          src
        } = this.context;
        if (src[end - 1] !== "'") errors.push(new PlainValue.YAMLSyntaxError(this, "Missing closing 'quote"));
        let str = "";
        for (let i = start + 1; i < end - 1; ++i) {
          const ch = src[i];
          if (ch === "\n") {
            if (PlainValue.Node.atDocumentBoundary(src, i + 1)) errors.push(new PlainValue.YAMLSemanticError(this, "Document boundary indicators are not allowed within string values"));
            const {
              fold,
              offset,
              error
            } = PlainValue.Node.foldNewline(src, i, indent);
            str += fold;
            i = offset;
            if (error) errors.push(new PlainValue.YAMLSemanticError(this, "Multi-line single-quoted string needs to be sufficiently indented"));
          } else if (ch === "'") {
            str += ch;
            i += 1;
            if (src[i] !== "'") errors.push(new PlainValue.YAMLSyntaxError(this, "Unescaped single quote? This should not happen."));
          } else if (ch === " " || ch === "	") {
            const wsStart = i;
            let next = src[i + 1];
            while (next === " " || next === "	") {
              i += 1;
              next = src[i + 1];
            }
            if (next !== "\n") str += i > wsStart ? src.slice(wsStart, i + 1) : ch;
          } else {
            str += ch;
          }
        }
        return errors.length > 0 ? {
          errors,
          str
        } : str;
      }
      /**
       * Parses a 'single quoted' value from the source
       *
       * @param {ParseContext} context
       * @param {number} start - Index of first character
       * @returns {number} - Index of the character after this scalar
       */
      parse(context, start) {
        this.context = context;
        const {
          src
        } = context;
        let offset = _QuoteSingle.endOfQuote(src, start + 1);
        this.valueRange = new PlainValue.Range(start, offset);
        offset = PlainValue.Node.endOfWhiteSpace(src, offset);
        offset = this.parseComment(offset);
        return offset;
      }
    };
    function createNewNode(type, props) {
      switch (type) {
        case PlainValue.Type.ALIAS:
          return new Alias(type, props);
        case PlainValue.Type.BLOCK_FOLDED:
        case PlainValue.Type.BLOCK_LITERAL:
          return new BlockValue(type, props);
        case PlainValue.Type.FLOW_MAP:
        case PlainValue.Type.FLOW_SEQ:
          return new FlowCollection(type, props);
        case PlainValue.Type.MAP_KEY:
        case PlainValue.Type.MAP_VALUE:
        case PlainValue.Type.SEQ_ITEM:
          return new CollectionItem(type, props);
        case PlainValue.Type.COMMENT:
        case PlainValue.Type.PLAIN:
          return new PlainValue.PlainValue(type, props);
        case PlainValue.Type.QUOTE_DOUBLE:
          return new QuoteDouble(type, props);
        case PlainValue.Type.QUOTE_SINGLE:
          return new QuoteSingle(type, props);
        /* istanbul ignore next */
        default:
          return null;
      }
    }
    var ParseContext = class _ParseContext {
      static parseType(src, offset, inFlow) {
        switch (src[offset]) {
          case "*":
            return PlainValue.Type.ALIAS;
          case ">":
            return PlainValue.Type.BLOCK_FOLDED;
          case "|":
            return PlainValue.Type.BLOCK_LITERAL;
          case "{":
            return PlainValue.Type.FLOW_MAP;
          case "[":
            return PlainValue.Type.FLOW_SEQ;
          case "?":
            return !inFlow && PlainValue.Node.atBlank(src, offset + 1, true) ? PlainValue.Type.MAP_KEY : PlainValue.Type.PLAIN;
          case ":":
            return !inFlow && PlainValue.Node.atBlank(src, offset + 1, true) ? PlainValue.Type.MAP_VALUE : PlainValue.Type.PLAIN;
          case "-":
            return !inFlow && PlainValue.Node.atBlank(src, offset + 1, true) ? PlainValue.Type.SEQ_ITEM : PlainValue.Type.PLAIN;
          case '"':
            return PlainValue.Type.QUOTE_DOUBLE;
          case "'":
            return PlainValue.Type.QUOTE_SINGLE;
          default:
            return PlainValue.Type.PLAIN;
        }
      }
      constructor(orig = {}, {
        atLineStart,
        inCollection,
        inFlow,
        indent,
        lineStart,
        parent
      } = {}) {
        PlainValue._defineProperty(this, "parseNode", (overlay, start) => {
          if (PlainValue.Node.atDocumentBoundary(this.src, start)) return null;
          const context = new _ParseContext(this, overlay);
          const {
            props,
            type,
            valueStart
          } = context.parseProps(start);
          const node = createNewNode(type, props);
          let offset = node.parse(context, valueStart);
          node.range = new PlainValue.Range(start, offset);
          if (offset <= start) {
            node.error = new Error(`Node#parse consumed no characters`);
            node.error.parseEnd = offset;
            node.error.source = node;
            node.range.end = start + 1;
          }
          if (context.nodeStartsCollection(node)) {
            if (!node.error && !context.atLineStart && context.parent.type === PlainValue.Type.DOCUMENT) {
              node.error = new PlainValue.YAMLSyntaxError(node, "Block collection must not have preceding content here (e.g. directives-end indicator)");
            }
            const collection = new Collection(node);
            offset = collection.parse(new _ParseContext(context), offset);
            collection.range = new PlainValue.Range(start, offset);
            return collection;
          }
          return node;
        });
        this.atLineStart = atLineStart != null ? atLineStart : orig.atLineStart || false;
        this.inCollection = inCollection != null ? inCollection : orig.inCollection || false;
        this.inFlow = inFlow != null ? inFlow : orig.inFlow || false;
        this.indent = indent != null ? indent : orig.indent;
        this.lineStart = lineStart != null ? lineStart : orig.lineStart;
        this.parent = parent != null ? parent : orig.parent || {};
        this.root = orig.root;
        this.src = orig.src;
      }
      nodeStartsCollection(node) {
        const {
          inCollection,
          inFlow,
          src
        } = this;
        if (inCollection || inFlow) return false;
        if (node instanceof CollectionItem) return true;
        let offset = node.range.end;
        if (src[offset] === "\n" || src[offset - 1] === "\n") return false;
        offset = PlainValue.Node.endOfWhiteSpace(src, offset);
        return src[offset] === ":";
      }
      // Anchor and tag are before type, which determines the node implementation
      // class; hence this intermediate step.
      parseProps(offset) {
        const {
          inFlow,
          parent,
          src
        } = this;
        const props = [];
        let lineHasProps = false;
        offset = this.atLineStart ? PlainValue.Node.endOfIndent(src, offset) : PlainValue.Node.endOfWhiteSpace(src, offset);
        let ch = src[offset];
        while (ch === PlainValue.Char.ANCHOR || ch === PlainValue.Char.COMMENT || ch === PlainValue.Char.TAG || ch === "\n") {
          if (ch === "\n") {
            let inEnd = offset;
            let lineStart;
            do {
              lineStart = inEnd + 1;
              inEnd = PlainValue.Node.endOfIndent(src, lineStart);
            } while (src[inEnd] === "\n");
            const indentDiff = inEnd - (lineStart + this.indent);
            const noIndicatorAsIndent = parent.type === PlainValue.Type.SEQ_ITEM && parent.context.atLineStart;
            if (src[inEnd] !== "#" && !PlainValue.Node.nextNodeIsIndented(src[inEnd], indentDiff, !noIndicatorAsIndent)) break;
            this.atLineStart = true;
            this.lineStart = lineStart;
            lineHasProps = false;
            offset = inEnd;
          } else if (ch === PlainValue.Char.COMMENT) {
            const end = PlainValue.Node.endOfLine(src, offset + 1);
            props.push(new PlainValue.Range(offset, end));
            offset = end;
          } else {
            let end = PlainValue.Node.endOfIdentifier(src, offset + 1);
            if (ch === PlainValue.Char.TAG && src[end] === "," && /^[a-zA-Z0-9-]+\.[a-zA-Z0-9-]+,\d\d\d\d(-\d\d){0,2}\/\S/.test(src.slice(offset + 1, end + 13))) {
              end = PlainValue.Node.endOfIdentifier(src, end + 5);
            }
            props.push(new PlainValue.Range(offset, end));
            lineHasProps = true;
            offset = PlainValue.Node.endOfWhiteSpace(src, end);
          }
          ch = src[offset];
        }
        if (lineHasProps && ch === ":" && PlainValue.Node.atBlank(src, offset + 1, true)) offset -= 1;
        const type = _ParseContext.parseType(src, offset, inFlow);
        return {
          props,
          type,
          valueStart: offset
        };
      }
      /**
       * Parses a node from the source
       * @param {ParseContext} overlay
       * @param {number} start - Index of first non-whitespace character for the node
       * @returns {?Node} - null if at a document boundary
       */
    };
    function parse(src) {
      const cr = [];
      if (src.indexOf("\r") !== -1) {
        src = src.replace(/\r\n?/g, (match, offset2) => {
          if (match.length > 1) cr.push(offset2);
          return "\n";
        });
      }
      const documents = [];
      let offset = 0;
      do {
        const doc = new Document();
        const context = new ParseContext({
          src
        });
        offset = doc.parse(context, offset);
        documents.push(doc);
      } while (offset < src.length);
      documents.setOrigRanges = () => {
        if (cr.length === 0) return false;
        for (let i = 1; i < cr.length; ++i) cr[i] -= i;
        let crOffset = 0;
        for (let i = 0; i < documents.length; ++i) {
          crOffset = documents[i].setOrigRanges(cr, crOffset);
        }
        cr.splice(0, cr.length);
        return true;
      };
      documents.toString = () => documents.join("...\n");
      return documents;
    }
    exports2.parse = parse;
  }
});

// node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/resolveSeq-d03cb037.js
var require_resolveSeq_d03cb037 = __commonJS({
  "node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/resolveSeq-d03cb037.js"(exports2) {
    "use strict";
    var PlainValue = require_PlainValue_ec8e588e();
    function addCommentBefore(str, indent, comment) {
      if (!comment) return str;
      const cc = comment.replace(/[\s\S]^/gm, `$&${indent}#`);
      return `#${cc}
${indent}${str}`;
    }
    function addComment(str, indent, comment) {
      return !comment ? str : comment.indexOf("\n") === -1 ? `${str} #${comment}` : `${str}
` + comment.replace(/^/gm, `${indent || ""}#`);
    }
    var Node = class {
    };
    function toJSON(value, arg, ctx) {
      if (Array.isArray(value)) return value.map((v, i) => toJSON(v, String(i), ctx));
      if (value && typeof value.toJSON === "function") {
        const anchor = ctx && ctx.anchors && ctx.anchors.get(value);
        if (anchor) ctx.onCreate = (res2) => {
          anchor.res = res2;
          delete ctx.onCreate;
        };
        const res = value.toJSON(arg, ctx);
        if (anchor && ctx.onCreate) ctx.onCreate(res);
        return res;
      }
      if ((!ctx || !ctx.keep) && typeof value === "bigint") return Number(value);
      return value;
    }
    var Scalar = class extends Node {
      constructor(value) {
        super();
        this.value = value;
      }
      toJSON(arg, ctx) {
        return ctx && ctx.keep ? this.value : toJSON(this.value, arg, ctx);
      }
      toString() {
        return String(this.value);
      }
    };
    function collectionFromPath(schema, path, value) {
      let v = value;
      for (let i = path.length - 1; i >= 0; --i) {
        const k = path[i];
        if (Number.isInteger(k) && k >= 0) {
          const a = [];
          a[k] = v;
          v = a;
        } else {
          const o = {};
          Object.defineProperty(o, k, {
            value: v,
            writable: true,
            enumerable: true,
            configurable: true
          });
          v = o;
        }
      }
      return schema.createNode(v, false);
    }
    var isEmptyPath = (path) => path == null || typeof path === "object" && path[Symbol.iterator]().next().done;
    var Collection = class _Collection extends Node {
      constructor(schema) {
        super();
        PlainValue._defineProperty(this, "items", []);
        this.schema = schema;
      }
      addIn(path, value) {
        if (isEmptyPath(path)) this.add(value);
        else {
          const [key, ...rest] = path;
          const node = this.get(key, true);
          if (node instanceof _Collection) node.addIn(rest, value);
          else if (node === void 0 && this.schema) this.set(key, collectionFromPath(this.schema, rest, value));
          else throw new Error(`Expected YAML collection at ${key}. Remaining path: ${rest}`);
        }
      }
      deleteIn([key, ...rest]) {
        if (rest.length === 0) return this.delete(key);
        const node = this.get(key, true);
        if (node instanceof _Collection) return node.deleteIn(rest);
        else throw new Error(`Expected YAML collection at ${key}. Remaining path: ${rest}`);
      }
      getIn([key, ...rest], keepScalar) {
        const node = this.get(key, true);
        if (rest.length === 0) return !keepScalar && node instanceof Scalar ? node.value : node;
        else return node instanceof _Collection ? node.getIn(rest, keepScalar) : void 0;
      }
      hasAllNullValues() {
        return this.items.every((node) => {
          if (!node || node.type !== "PAIR") return false;
          const n = node.value;
          return n == null || n instanceof Scalar && n.value == null && !n.commentBefore && !n.comment && !n.tag;
        });
      }
      hasIn([key, ...rest]) {
        if (rest.length === 0) return this.has(key);
        const node = this.get(key, true);
        return node instanceof _Collection ? node.hasIn(rest) : false;
      }
      setIn([key, ...rest], value) {
        if (rest.length === 0) {
          this.set(key, value);
        } else {
          const node = this.get(key, true);
          if (node instanceof _Collection) node.setIn(rest, value);
          else if (node === void 0 && this.schema) this.set(key, collectionFromPath(this.schema, rest, value));
          else throw new Error(`Expected YAML collection at ${key}. Remaining path: ${rest}`);
        }
      }
      // overridden in implementations
      /* istanbul ignore next */
      toJSON() {
        return null;
      }
      toString(ctx, {
        blockItem,
        flowChars,
        isMap,
        itemIndent
      }, onComment, onChompKeep) {
        const {
          indent,
          indentStep,
          stringify
        } = ctx;
        const inFlow = this.type === PlainValue.Type.FLOW_MAP || this.type === PlainValue.Type.FLOW_SEQ || ctx.inFlow;
        if (inFlow) itemIndent += indentStep;
        const allNullValues = isMap && this.hasAllNullValues();
        ctx = Object.assign({}, ctx, {
          allNullValues,
          indent: itemIndent,
          inFlow,
          type: null
        });
        let chompKeep = false;
        let hasItemWithNewLine = false;
        const nodes = this.items.reduce((nodes2, item, i) => {
          let comment;
          if (item) {
            if (!chompKeep && item.spaceBefore) nodes2.push({
              type: "comment",
              str: ""
            });
            if (item.commentBefore) item.commentBefore.match(/^.*$/gm).forEach((line) => {
              nodes2.push({
                type: "comment",
                str: `#${line}`
              });
            });
            if (item.comment) comment = item.comment;
            if (inFlow && (!chompKeep && item.spaceBefore || item.commentBefore || item.comment || item.key && (item.key.commentBefore || item.key.comment) || item.value && (item.value.commentBefore || item.value.comment))) hasItemWithNewLine = true;
          }
          chompKeep = false;
          let str2 = stringify(item, ctx, () => comment = null, () => chompKeep = true);
          if (inFlow && !hasItemWithNewLine && str2.includes("\n")) hasItemWithNewLine = true;
          if (inFlow && i < this.items.length - 1) str2 += ",";
          str2 = addComment(str2, itemIndent, comment);
          if (chompKeep && (comment || inFlow)) chompKeep = false;
          nodes2.push({
            type: "item",
            str: str2
          });
          return nodes2;
        }, []);
        let str;
        if (nodes.length === 0) {
          str = flowChars.start + flowChars.end;
        } else if (inFlow) {
          const {
            start,
            end
          } = flowChars;
          const strings = nodes.map((n) => n.str);
          if (hasItemWithNewLine || strings.reduce((sum, str2) => sum + str2.length + 2, 2) > _Collection.maxFlowStringSingleLineLength) {
            str = start;
            for (const s of strings) {
              str += s ? `
${indentStep}${indent}${s}` : "\n";
            }
            str += `
${indent}${end}`;
          } else {
            str = `${start} ${strings.join(" ")} ${end}`;
          }
        } else {
          const strings = nodes.map(blockItem);
          str = strings.shift();
          for (const s of strings) str += s ? `
${indent}${s}` : "\n";
        }
        if (this.comment) {
          str += "\n" + this.comment.replace(/^/gm, `${indent}#`);
          if (onComment) onComment();
        } else if (chompKeep && onChompKeep) onChompKeep();
        return str;
      }
    };
    PlainValue._defineProperty(Collection, "maxFlowStringSingleLineLength", 60);
    function asItemIndex(key) {
      let idx = key instanceof Scalar ? key.value : key;
      if (idx && typeof idx === "string") idx = Number(idx);
      return Number.isInteger(idx) && idx >= 0 ? idx : null;
    }
    var YAMLSeq = class extends Collection {
      add(value) {
        this.items.push(value);
      }
      delete(key) {
        const idx = asItemIndex(key);
        if (typeof idx !== "number") return false;
        const del = this.items.splice(idx, 1);
        return del.length > 0;
      }
      get(key, keepScalar) {
        const idx = asItemIndex(key);
        if (typeof idx !== "number") return void 0;
        const it = this.items[idx];
        return !keepScalar && it instanceof Scalar ? it.value : it;
      }
      has(key) {
        const idx = asItemIndex(key);
        return typeof idx === "number" && idx < this.items.length;
      }
      set(key, value) {
        const idx = asItemIndex(key);
        if (typeof idx !== "number") throw new Error(`Expected a valid index, not ${key}.`);
        this.items[idx] = value;
      }
      toJSON(_, ctx) {
        const seq = [];
        if (ctx && ctx.onCreate) ctx.onCreate(seq);
        let i = 0;
        for (const item of this.items) seq.push(toJSON(item, String(i++), ctx));
        return seq;
      }
      toString(ctx, onComment, onChompKeep) {
        if (!ctx) return JSON.stringify(this);
        return super.toString(ctx, {
          blockItem: (n) => n.type === "comment" ? n.str : `- ${n.str}`,
          flowChars: {
            start: "[",
            end: "]"
          },
          isMap: false,
          itemIndent: (ctx.indent || "") + "  "
        }, onComment, onChompKeep);
      }
    };
    var stringifyKey = (key, jsKey, ctx) => {
      if (jsKey === null) return "";
      if (typeof jsKey !== "object") return String(jsKey);
      if (key instanceof Node && ctx && ctx.doc) return key.toString({
        anchors: /* @__PURE__ */ Object.create(null),
        doc: ctx.doc,
        indent: "",
        indentStep: ctx.indentStep,
        inFlow: true,
        inStringifyKey: true,
        stringify: ctx.stringify
      });
      return JSON.stringify(jsKey);
    };
    var Pair = class _Pair extends Node {
      constructor(key, value = null) {
        super();
        this.key = key;
        this.value = value;
        this.type = _Pair.Type.PAIR;
      }
      get commentBefore() {
        return this.key instanceof Node ? this.key.commentBefore : void 0;
      }
      set commentBefore(cb) {
        if (this.key == null) this.key = new Scalar(null);
        if (this.key instanceof Node) this.key.commentBefore = cb;
        else {
          const msg = "Pair.commentBefore is an alias for Pair.key.commentBefore. To set it, the key must be a Node.";
          throw new Error(msg);
        }
      }
      addToJSMap(ctx, map) {
        const key = toJSON(this.key, "", ctx);
        if (map instanceof Map) {
          const value = toJSON(this.value, key, ctx);
          map.set(key, value);
        } else if (map instanceof Set) {
          map.add(key);
        } else {
          const stringKey = stringifyKey(this.key, key, ctx);
          const value = toJSON(this.value, stringKey, ctx);
          if (stringKey in map) Object.defineProperty(map, stringKey, {
            value,
            writable: true,
            enumerable: true,
            configurable: true
          });
          else map[stringKey] = value;
        }
        return map;
      }
      toJSON(_, ctx) {
        const pair = ctx && ctx.mapAsMap ? /* @__PURE__ */ new Map() : {};
        return this.addToJSMap(ctx, pair);
      }
      toString(ctx, onComment, onChompKeep) {
        if (!ctx || !ctx.doc) return JSON.stringify(this);
        const {
          indent: indentSize,
          indentSeq,
          simpleKeys
        } = ctx.doc.options;
        let {
          key,
          value
        } = this;
        let keyComment = key instanceof Node && key.comment;
        if (simpleKeys) {
          if (keyComment) {
            throw new Error("With simple keys, key nodes cannot have comments");
          }
          if (key instanceof Collection) {
            const msg = "With simple keys, collection cannot be used as a key value";
            throw new Error(msg);
          }
        }
        let explicitKey = !simpleKeys && (!key || keyComment || (key instanceof Node ? key instanceof Collection || key.type === PlainValue.Type.BLOCK_FOLDED || key.type === PlainValue.Type.BLOCK_LITERAL : typeof key === "object"));
        const {
          doc,
          indent,
          indentStep,
          stringify
        } = ctx;
        ctx = Object.assign({}, ctx, {
          implicitKey: !explicitKey,
          indent: indent + indentStep
        });
        let chompKeep = false;
        let str = stringify(key, ctx, () => keyComment = null, () => chompKeep = true);
        str = addComment(str, ctx.indent, keyComment);
        if (!explicitKey && str.length > 1024) {
          if (simpleKeys) throw new Error("With simple keys, single line scalar must not span more than 1024 characters");
          explicitKey = true;
        }
        if (ctx.allNullValues && !simpleKeys) {
          if (this.comment) {
            str = addComment(str, ctx.indent, this.comment);
            if (onComment) onComment();
          } else if (chompKeep && !keyComment && onChompKeep) onChompKeep();
          return ctx.inFlow && !explicitKey ? str : `? ${str}`;
        }
        str = explicitKey ? `? ${str}
${indent}:` : `${str}:`;
        if (this.comment) {
          str = addComment(str, ctx.indent, this.comment);
          if (onComment) onComment();
        }
        let vcb = "";
        let valueComment = null;
        if (value instanceof Node) {
          if (value.spaceBefore) vcb = "\n";
          if (value.commentBefore) {
            const cs = value.commentBefore.replace(/^/gm, `${ctx.indent}#`);
            vcb += `
${cs}`;
          }
          valueComment = value.comment;
        } else if (value && typeof value === "object") {
          value = doc.schema.createNode(value, true);
        }
        ctx.implicitKey = false;
        if (!explicitKey && !this.comment && value instanceof Scalar) ctx.indentAtStart = str.length + 1;
        chompKeep = false;
        if (!indentSeq && indentSize >= 2 && !ctx.inFlow && !explicitKey && value instanceof YAMLSeq && value.type !== PlainValue.Type.FLOW_SEQ && !value.tag && !doc.anchors.getName(value)) {
          ctx.indent = ctx.indent.substr(2);
        }
        const valueStr = stringify(value, ctx, () => valueComment = null, () => chompKeep = true);
        let ws = " ";
        if (vcb || this.comment) {
          ws = `${vcb}
${ctx.indent}`;
        } else if (!explicitKey && value instanceof Collection) {
          const flow = valueStr[0] === "[" || valueStr[0] === "{";
          if (!flow || valueStr.includes("\n")) ws = `
${ctx.indent}`;
        } else if (valueStr[0] === "\n") ws = "";
        if (chompKeep && !valueComment && onChompKeep) onChompKeep();
        return addComment(str + ws + valueStr, ctx.indent, valueComment);
      }
    };
    PlainValue._defineProperty(Pair, "Type", {
      PAIR: "PAIR",
      MERGE_PAIR: "MERGE_PAIR"
    });
    var getAliasCount = (node, anchors) => {
      if (node instanceof Alias) {
        const anchor = anchors.get(node.source);
        return anchor.count * anchor.aliasCount;
      } else if (node instanceof Collection) {
        let count = 0;
        for (const item of node.items) {
          const c = getAliasCount(item, anchors);
          if (c > count) count = c;
        }
        return count;
      } else if (node instanceof Pair) {
        const kc = getAliasCount(node.key, anchors);
        const vc = getAliasCount(node.value, anchors);
        return Math.max(kc, vc);
      }
      return 1;
    };
    var Alias = class _Alias extends Node {
      static stringify({
        range,
        source
      }, {
        anchors,
        doc,
        implicitKey,
        inStringifyKey
      }) {
        let anchor = Object.keys(anchors).find((a) => anchors[a] === source);
        if (!anchor && inStringifyKey) anchor = doc.anchors.getName(source) || doc.anchors.newName();
        if (anchor) return `*${anchor}${implicitKey ? " " : ""}`;
        const msg = doc.anchors.getName(source) ? "Alias node must be after source node" : "Source node not found for alias node";
        throw new Error(`${msg} [${range}]`);
      }
      constructor(source) {
        super();
        this.source = source;
        this.type = PlainValue.Type.ALIAS;
      }
      set tag(t) {
        throw new Error("Alias nodes cannot have tags");
      }
      toJSON(arg, ctx) {
        if (!ctx) return toJSON(this.source, arg, ctx);
        const {
          anchors,
          maxAliasCount
        } = ctx;
        const anchor = anchors.get(this.source);
        if (!anchor || anchor.res === void 0) {
          const msg = "This should not happen: Alias anchor was not resolved?";
          if (this.cstNode) throw new PlainValue.YAMLReferenceError(this.cstNode, msg);
          else throw new ReferenceError(msg);
        }
        if (maxAliasCount >= 0) {
          anchor.count += 1;
          if (anchor.aliasCount === 0) anchor.aliasCount = getAliasCount(this.source, anchors);
          if (anchor.count * anchor.aliasCount > maxAliasCount) {
            const msg = "Excessive alias count indicates a resource exhaustion attack";
            if (this.cstNode) throw new PlainValue.YAMLReferenceError(this.cstNode, msg);
            else throw new ReferenceError(msg);
          }
        }
        return anchor.res;
      }
      // Only called when stringifying an alias mapping key while constructing
      // Object output.
      toString(ctx) {
        return _Alias.stringify(this, ctx);
      }
    };
    PlainValue._defineProperty(Alias, "default", true);
    function findPair(items, key) {
      const k = key instanceof Scalar ? key.value : key;
      for (const it of items) {
        if (it instanceof Pair) {
          if (it.key === key || it.key === k) return it;
          if (it.key && it.key.value === k) return it;
        }
      }
      return void 0;
    }
    var YAMLMap = class extends Collection {
      add(pair, overwrite) {
        if (!pair) pair = new Pair(pair);
        else if (!(pair instanceof Pair)) pair = new Pair(pair.key || pair, pair.value);
        const prev = findPair(this.items, pair.key);
        const sortEntries = this.schema && this.schema.sortMapEntries;
        if (prev) {
          if (overwrite) prev.value = pair.value;
          else throw new Error(`Key ${pair.key} already set`);
        } else if (sortEntries) {
          const i = this.items.findIndex((item) => sortEntries(pair, item) < 0);
          if (i === -1) this.items.push(pair);
          else this.items.splice(i, 0, pair);
        } else {
          this.items.push(pair);
        }
      }
      delete(key) {
        const it = findPair(this.items, key);
        if (!it) return false;
        const del = this.items.splice(this.items.indexOf(it), 1);
        return del.length > 0;
      }
      get(key, keepScalar) {
        const it = findPair(this.items, key);
        const node = it && it.value;
        return !keepScalar && node instanceof Scalar ? node.value : node;
      }
      has(key) {
        return !!findPair(this.items, key);
      }
      set(key, value) {
        this.add(new Pair(key, value), true);
      }
      /**
       * @param {*} arg ignored
       * @param {*} ctx Conversion context, originally set in Document#toJSON()
       * @param {Class} Type If set, forces the returned collection type
       * @returns {*} Instance of Type, Map, or Object
       */
      toJSON(_, ctx, Type) {
        const map = Type ? new Type() : ctx && ctx.mapAsMap ? /* @__PURE__ */ new Map() : {};
        if (ctx && ctx.onCreate) ctx.onCreate(map);
        for (const item of this.items) item.addToJSMap(ctx, map);
        return map;
      }
      toString(ctx, onComment, onChompKeep) {
        if (!ctx) return JSON.stringify(this);
        for (const item of this.items) {
          if (!(item instanceof Pair)) throw new Error(`Map items must all be pairs; found ${JSON.stringify(item)} instead`);
        }
        return super.toString(ctx, {
          blockItem: (n) => n.str,
          flowChars: {
            start: "{",
            end: "}"
          },
          isMap: true,
          itemIndent: ctx.indent || ""
        }, onComment, onChompKeep);
      }
    };
    var MERGE_KEY = "<<";
    var Merge = class extends Pair {
      constructor(pair) {
        if (pair instanceof Pair) {
          let seq = pair.value;
          if (!(seq instanceof YAMLSeq)) {
            seq = new YAMLSeq();
            seq.items.push(pair.value);
            seq.range = pair.value.range;
          }
          super(pair.key, seq);
          this.range = pair.range;
        } else {
          super(new Scalar(MERGE_KEY), new YAMLSeq());
        }
        this.type = Pair.Type.MERGE_PAIR;
      }
      // If the value associated with a merge key is a single mapping node, each of
      // its key/value pairs is inserted into the current mapping, unless the key
      // already exists in it. If the value associated with the merge key is a
      // sequence, then this sequence is expected to contain mapping nodes and each
      // of these nodes is merged in turn according to its order in the sequence.
      // Keys in mapping nodes earlier in the sequence override keys specified in
      // later mapping nodes. -- http://yaml.org/type/merge.html
      addToJSMap(ctx, map) {
        for (const {
          source
        } of this.value.items) {
          if (!(source instanceof YAMLMap)) throw new Error("Merge sources must be maps");
          const srcMap = source.toJSON(null, ctx, Map);
          for (const [key, value] of srcMap) {
            if (map instanceof Map) {
              if (!map.has(key)) map.set(key, value);
            } else if (map instanceof Set) {
              map.add(key);
            } else if (!Object.prototype.hasOwnProperty.call(map, key)) {
              Object.defineProperty(map, key, {
                value,
                writable: true,
                enumerable: true,
                configurable: true
              });
            }
          }
        }
        return map;
      }
      toString(ctx, onComment) {
        const seq = this.value;
        if (seq.items.length > 1) return super.toString(ctx, onComment);
        this.value = seq.items[0];
        const str = super.toString(ctx, onComment);
        this.value = seq;
        return str;
      }
    };
    var binaryOptions = {
      defaultType: PlainValue.Type.BLOCK_LITERAL,
      lineWidth: 76
    };
    var boolOptions = {
      trueStr: "true",
      falseStr: "false"
    };
    var intOptions = {
      asBigInt: false
    };
    var nullOptions = {
      nullStr: "null"
    };
    var strOptions = {
      defaultType: PlainValue.Type.PLAIN,
      doubleQuoted: {
        jsonEncoding: false,
        minMultiLineLength: 40
      },
      fold: {
        lineWidth: 80,
        minContentWidth: 20
      }
    };
    function resolveScalar(str, tags, scalarFallback) {
      for (const {
        format,
        test,
        resolve
      } of tags) {
        if (test) {
          const match = str.match(test);
          if (match) {
            let res = resolve.apply(null, match);
            if (!(res instanceof Scalar)) res = new Scalar(res);
            if (format) res.format = format;
            return res;
          }
        }
      }
      if (scalarFallback) str = scalarFallback(str);
      return new Scalar(str);
    }
    var FOLD_FLOW = "flow";
    var FOLD_BLOCK = "block";
    var FOLD_QUOTED = "quoted";
    var consumeMoreIndentedLines = (text, i) => {
      let ch = text[i + 1];
      while (ch === " " || ch === "	") {
        do {
          ch = text[i += 1];
        } while (ch && ch !== "\n");
        ch = text[i + 1];
      }
      return i;
    };
    function foldFlowLines(text, indent, mode, {
      indentAtStart,
      lineWidth = 80,
      minContentWidth = 20,
      onFold,
      onOverflow
    }) {
      if (!lineWidth || lineWidth < 0) return text;
      const endStep = Math.max(1 + minContentWidth, 1 + lineWidth - indent.length);
      if (text.length <= endStep) return text;
      const folds = [];
      const escapedFolds = {};
      let end = lineWidth - indent.length;
      if (typeof indentAtStart === "number") {
        if (indentAtStart > lineWidth - Math.max(2, minContentWidth)) folds.push(0);
        else end = lineWidth - indentAtStart;
      }
      let split = void 0;
      let prev = void 0;
      let overflow = false;
      let i = -1;
      let escStart = -1;
      let escEnd = -1;
      if (mode === FOLD_BLOCK) {
        i = consumeMoreIndentedLines(text, i);
        if (i !== -1) end = i + endStep;
      }
      for (let ch; ch = text[i += 1]; ) {
        if (mode === FOLD_QUOTED && ch === "\\") {
          escStart = i;
          switch (text[i + 1]) {
            case "x":
              i += 3;
              break;
            case "u":
              i += 5;
              break;
            case "U":
              i += 9;
              break;
            default:
              i += 1;
          }
          escEnd = i;
        }
        if (ch === "\n") {
          if (mode === FOLD_BLOCK) i = consumeMoreIndentedLines(text, i);
          end = i + endStep;
          split = void 0;
        } else {
          if (ch === " " && prev && prev !== " " && prev !== "\n" && prev !== "	") {
            const next = text[i + 1];
            if (next && next !== " " && next !== "\n" && next !== "	") split = i;
          }
          if (i >= end) {
            if (split) {
              folds.push(split);
              end = split + endStep;
              split = void 0;
            } else if (mode === FOLD_QUOTED) {
              while (prev === " " || prev === "	") {
                prev = ch;
                ch = text[i += 1];
                overflow = true;
              }
              const j = i > escEnd + 1 ? i - 2 : escStart - 1;
              if (escapedFolds[j]) return text;
              folds.push(j);
              escapedFolds[j] = true;
              end = j + endStep;
              split = void 0;
            } else {
              overflow = true;
            }
          }
        }
        prev = ch;
      }
      if (overflow && onOverflow) onOverflow();
      if (folds.length === 0) return text;
      if (onFold) onFold();
      let res = text.slice(0, folds[0]);
      for (let i2 = 0; i2 < folds.length; ++i2) {
        const fold = folds[i2];
        const end2 = folds[i2 + 1] || text.length;
        if (fold === 0) res = `
${indent}${text.slice(0, end2)}`;
        else {
          if (mode === FOLD_QUOTED && escapedFolds[fold]) res += `${text[fold]}\\`;
          res += `
${indent}${text.slice(fold + 1, end2)}`;
        }
      }
      return res;
    }
    var getFoldOptions = ({
      indentAtStart
    }) => indentAtStart ? Object.assign({
      indentAtStart
    }, strOptions.fold) : strOptions.fold;
    var containsDocumentMarker = (str) => /^(%|---|\.\.\.)/m.test(str);
    function lineLengthOverLimit(str, lineWidth, indentLength) {
      if (!lineWidth || lineWidth < 0) return false;
      const limit = lineWidth - indentLength;
      const strLen = str.length;
      if (strLen <= limit) return false;
      for (let i = 0, start = 0; i < strLen; ++i) {
        if (str[i] === "\n") {
          if (i - start > limit) return true;
          start = i + 1;
          if (strLen - start <= limit) return false;
        }
      }
      return true;
    }
    function doubleQuotedString(value, ctx) {
      const {
        implicitKey
      } = ctx;
      const {
        jsonEncoding,
        minMultiLineLength
      } = strOptions.doubleQuoted;
      const json = JSON.stringify(value);
      if (jsonEncoding) return json;
      const indent = ctx.indent || (containsDocumentMarker(value) ? "  " : "");
      let str = "";
      let start = 0;
      for (let i = 0, ch = json[i]; ch; ch = json[++i]) {
        if (ch === " " && json[i + 1] === "\\" && json[i + 2] === "n") {
          str += json.slice(start, i) + "\\ ";
          i += 1;
          start = i;
          ch = "\\";
        }
        if (ch === "\\") switch (json[i + 1]) {
          case "u":
            {
              str += json.slice(start, i);
              const code = json.substr(i + 2, 4);
              switch (code) {
                case "0000":
                  str += "\\0";
                  break;
                case "0007":
                  str += "\\a";
                  break;
                case "000b":
                  str += "\\v";
                  break;
                case "001b":
                  str += "\\e";
                  break;
                case "0085":
                  str += "\\N";
                  break;
                case "00a0":
                  str += "\\_";
                  break;
                case "2028":
                  str += "\\L";
                  break;
                case "2029":
                  str += "\\P";
                  break;
                default:
                  if (code.substr(0, 2) === "00") str += "\\x" + code.substr(2);
                  else str += json.substr(i, 6);
              }
              i += 5;
              start = i + 1;
            }
            break;
          case "n":
            if (implicitKey || json[i + 2] === '"' || json.length < minMultiLineLength) {
              i += 1;
            } else {
              str += json.slice(start, i) + "\n\n";
              while (json[i + 2] === "\\" && json[i + 3] === "n" && json[i + 4] !== '"') {
                str += "\n";
                i += 2;
              }
              str += indent;
              if (json[i + 2] === " ") str += "\\";
              i += 1;
              start = i + 1;
            }
            break;
          default:
            i += 1;
        }
      }
      str = start ? str + json.slice(start) : json;
      return implicitKey ? str : foldFlowLines(str, indent, FOLD_QUOTED, getFoldOptions(ctx));
    }
    function singleQuotedString(value, ctx) {
      if (ctx.implicitKey) {
        if (/\n/.test(value)) return doubleQuotedString(value, ctx);
      } else {
        if (/[ \t]\n|\n[ \t]/.test(value)) return doubleQuotedString(value, ctx);
      }
      const indent = ctx.indent || (containsDocumentMarker(value) ? "  " : "");
      const res = "'" + value.replace(/'/g, "''").replace(/\n+/g, `$&
${indent}`) + "'";
      return ctx.implicitKey ? res : foldFlowLines(res, indent, FOLD_FLOW, getFoldOptions(ctx));
    }
    function blockString({
      comment,
      type,
      value
    }, ctx, onComment, onChompKeep) {
      if (/\n[\t ]+$/.test(value) || /^\s*$/.test(value)) {
        return doubleQuotedString(value, ctx);
      }
      const indent = ctx.indent || (ctx.forceBlockIndent || containsDocumentMarker(value) ? "  " : "");
      const indentSize = indent ? "2" : "1";
      const literal = type === PlainValue.Type.BLOCK_FOLDED ? false : type === PlainValue.Type.BLOCK_LITERAL ? true : !lineLengthOverLimit(value, strOptions.fold.lineWidth, indent.length);
      let header = literal ? "|" : ">";
      if (!value) return header + "\n";
      let wsStart = "";
      let wsEnd = "";
      value = value.replace(/[\n\t ]*$/, (ws) => {
        const n = ws.indexOf("\n");
        if (n === -1) {
          header += "-";
        } else if (value === ws || n !== ws.length - 1) {
          header += "+";
          if (onChompKeep) onChompKeep();
        }
        wsEnd = ws.replace(/\n$/, "");
        return "";
      }).replace(/^[\n ]*/, (ws) => {
        if (ws.indexOf(" ") !== -1) header += indentSize;
        const m = ws.match(/ +$/);
        if (m) {
          wsStart = ws.slice(0, -m[0].length);
          return m[0];
        } else {
          wsStart = ws;
          return "";
        }
      });
      if (wsEnd) wsEnd = wsEnd.replace(/\n+(?!\n|$)/g, `$&${indent}`);
      if (wsStart) wsStart = wsStart.replace(/\n+/g, `$&${indent}`);
      if (comment) {
        header += " #" + comment.replace(/ ?[\r\n]+/g, " ");
        if (onComment) onComment();
      }
      if (!value) return `${header}${indentSize}
${indent}${wsEnd}`;
      if (literal) {
        value = value.replace(/\n+/g, `$&${indent}`);
        return `${header}
${indent}${wsStart}${value}${wsEnd}`;
      }
      value = value.replace(/\n+/g, "\n$&").replace(/(?:^|\n)([\t ].*)(?:([\n\t ]*)\n(?![\n\t ]))?/g, "$1$2").replace(/\n+/g, `$&${indent}`);
      const body = foldFlowLines(`${wsStart}${value}${wsEnd}`, indent, FOLD_BLOCK, strOptions.fold);
      return `${header}
${indent}${body}`;
    }
    function plainString(item, ctx, onComment, onChompKeep) {
      const {
        comment,
        type,
        value
      } = item;
      const {
        actualString,
        implicitKey,
        indent,
        inFlow
      } = ctx;
      if (implicitKey && /[\n[\]{},]/.test(value) || inFlow && /[[\]{},]/.test(value)) {
        return doubleQuotedString(value, ctx);
      }
      if (!value || /^[\n\t ,[\]{}#&*!|>'"%@`]|^[?-]$|^[?-][ \t]|[\n:][ \t]|[ \t]\n|[\n\t ]#|[\n\t :]$/.test(value)) {
        return implicitKey || inFlow || value.indexOf("\n") === -1 ? value.indexOf('"') !== -1 && value.indexOf("'") === -1 ? singleQuotedString(value, ctx) : doubleQuotedString(value, ctx) : blockString(item, ctx, onComment, onChompKeep);
      }
      if (!implicitKey && !inFlow && type !== PlainValue.Type.PLAIN && value.indexOf("\n") !== -1) {
        return blockString(item, ctx, onComment, onChompKeep);
      }
      if (indent === "" && containsDocumentMarker(value)) {
        ctx.forceBlockIndent = true;
        return blockString(item, ctx, onComment, onChompKeep);
      }
      const str = value.replace(/\n+/g, `$&
${indent}`);
      if (actualString) {
        const {
          tags
        } = ctx.doc.schema;
        const resolved = resolveScalar(str, tags, tags.scalarFallback).value;
        if (typeof resolved !== "string") return doubleQuotedString(value, ctx);
      }
      const body = implicitKey ? str : foldFlowLines(str, indent, FOLD_FLOW, getFoldOptions(ctx));
      if (comment && !inFlow && (body.indexOf("\n") !== -1 || comment.indexOf("\n") !== -1)) {
        if (onComment) onComment();
        return addCommentBefore(body, indent, comment);
      }
      return body;
    }
    function stringifyString(item, ctx, onComment, onChompKeep) {
      const {
        defaultType
      } = strOptions;
      const {
        implicitKey,
        inFlow
      } = ctx;
      let {
        type,
        value
      } = item;
      if (typeof value !== "string") {
        value = String(value);
        item = Object.assign({}, item, {
          value
        });
      }
      const _stringify = (_type) => {
        switch (_type) {
          case PlainValue.Type.BLOCK_FOLDED:
          case PlainValue.Type.BLOCK_LITERAL:
            return blockString(item, ctx, onComment, onChompKeep);
          case PlainValue.Type.QUOTE_DOUBLE:
            return doubleQuotedString(value, ctx);
          case PlainValue.Type.QUOTE_SINGLE:
            return singleQuotedString(value, ctx);
          case PlainValue.Type.PLAIN:
            return plainString(item, ctx, onComment, onChompKeep);
          default:
            return null;
        }
      };
      if (type !== PlainValue.Type.QUOTE_DOUBLE && /[\x00-\x08\x0b-\x1f\x7f-\x9f]/.test(value)) {
        type = PlainValue.Type.QUOTE_DOUBLE;
      } else if ((implicitKey || inFlow) && (type === PlainValue.Type.BLOCK_FOLDED || type === PlainValue.Type.BLOCK_LITERAL)) {
        type = PlainValue.Type.QUOTE_DOUBLE;
      }
      let res = _stringify(type);
      if (res === null) {
        res = _stringify(defaultType);
        if (res === null) throw new Error(`Unsupported default string type ${defaultType}`);
      }
      return res;
    }
    function stringifyNumber({
      format,
      minFractionDigits,
      tag,
      value
    }) {
      if (typeof value === "bigint") return String(value);
      if (!isFinite(value)) return isNaN(value) ? ".nan" : value < 0 ? "-.inf" : ".inf";
      let n = JSON.stringify(value);
      if (!format && minFractionDigits && (!tag || tag === "tag:yaml.org,2002:float") && /^\d/.test(n)) {
        let i = n.indexOf(".");
        if (i < 0) {
          i = n.length;
          n += ".";
        }
        let d = minFractionDigits - (n.length - i - 1);
        while (d-- > 0) n += "0";
      }
      return n;
    }
    function checkFlowCollectionEnd(errors, cst) {
      let char, name;
      switch (cst.type) {
        case PlainValue.Type.FLOW_MAP:
          char = "}";
          name = "flow map";
          break;
        case PlainValue.Type.FLOW_SEQ:
          char = "]";
          name = "flow sequence";
          break;
        default:
          errors.push(new PlainValue.YAMLSemanticError(cst, "Not a flow collection!?"));
          return;
      }
      let lastItem;
      for (let i = cst.items.length - 1; i >= 0; --i) {
        const item = cst.items[i];
        if (!item || item.type !== PlainValue.Type.COMMENT) {
          lastItem = item;
          break;
        }
      }
      if (lastItem && lastItem.char !== char) {
        const msg = `Expected ${name} to end with ${char}`;
        let err;
        if (typeof lastItem.offset === "number") {
          err = new PlainValue.YAMLSemanticError(cst, msg);
          err.offset = lastItem.offset + 1;
        } else {
          err = new PlainValue.YAMLSemanticError(lastItem, msg);
          if (lastItem.range && lastItem.range.end) err.offset = lastItem.range.end - lastItem.range.start;
        }
        errors.push(err);
      }
    }
    function checkFlowCommentSpace(errors, comment) {
      const prev = comment.context.src[comment.range.start - 1];
      if (prev !== "\n" && prev !== "	" && prev !== " ") {
        const msg = "Comments must be separated from other tokens by white space characters";
        errors.push(new PlainValue.YAMLSemanticError(comment, msg));
      }
    }
    function getLongKeyError(source, key) {
      const sk = String(key);
      const k = sk.substr(0, 8) + "..." + sk.substr(-8);
      return new PlainValue.YAMLSemanticError(source, `The "${k}" key is too long`);
    }
    function resolveComments(collection, comments) {
      for (const {
        afterKey,
        before,
        comment
      } of comments) {
        let item = collection.items[before];
        if (!item) {
          if (comment !== void 0) {
            if (collection.comment) collection.comment += "\n" + comment;
            else collection.comment = comment;
          }
        } else {
          if (afterKey && item.value) item = item.value;
          if (comment === void 0) {
            if (afterKey || !item.commentBefore) item.spaceBefore = true;
          } else {
            if (item.commentBefore) item.commentBefore += "\n" + comment;
            else item.commentBefore = comment;
          }
        }
      }
    }
    function resolveString(doc, node) {
      const res = node.strValue;
      if (!res) return "";
      if (typeof res === "string") return res;
      res.errors.forEach((error) => {
        if (!error.source) error.source = node;
        doc.errors.push(error);
      });
      return res.str;
    }
    function resolveTagHandle(doc, node) {
      const {
        handle,
        suffix
      } = node.tag;
      let prefix = doc.tagPrefixes.find((p) => p.handle === handle);
      if (!prefix) {
        const dtp = doc.getDefaults().tagPrefixes;
        if (dtp) prefix = dtp.find((p) => p.handle === handle);
        if (!prefix) throw new PlainValue.YAMLSemanticError(node, `The ${handle} tag handle is non-default and was not declared.`);
      }
      if (!suffix) throw new PlainValue.YAMLSemanticError(node, `The ${handle} tag has no suffix.`);
      if (handle === "!" && (doc.version || doc.options.version) === "1.0") {
        if (suffix[0] === "^") {
          doc.warnings.push(new PlainValue.YAMLWarning(node, "YAML 1.0 ^ tag expansion is not supported"));
          return suffix;
        }
        if (/[:/]/.test(suffix)) {
          const vocab = suffix.match(/^([a-z0-9-]+)\/(.*)/i);
          return vocab ? `tag:${vocab[1]}.yaml.org,2002:${vocab[2]}` : `tag:${suffix}`;
        }
      }
      return prefix.prefix + decodeURIComponent(suffix);
    }
    function resolveTagName(doc, node) {
      const {
        tag,
        type
      } = node;
      let nonSpecific = false;
      if (tag) {
        const {
          handle,
          suffix,
          verbatim
        } = tag;
        if (verbatim) {
          if (verbatim !== "!" && verbatim !== "!!") return verbatim;
          const msg = `Verbatim tags aren't resolved, so ${verbatim} is invalid.`;
          doc.errors.push(new PlainValue.YAMLSemanticError(node, msg));
        } else if (handle === "!" && !suffix) {
          nonSpecific = true;
        } else {
          try {
            return resolveTagHandle(doc, node);
          } catch (error) {
            doc.errors.push(error);
          }
        }
      }
      switch (type) {
        case PlainValue.Type.BLOCK_FOLDED:
        case PlainValue.Type.BLOCK_LITERAL:
        case PlainValue.Type.QUOTE_DOUBLE:
        case PlainValue.Type.QUOTE_SINGLE:
          return PlainValue.defaultTags.STR;
        case PlainValue.Type.FLOW_MAP:
        case PlainValue.Type.MAP:
          return PlainValue.defaultTags.MAP;
        case PlainValue.Type.FLOW_SEQ:
        case PlainValue.Type.SEQ:
          return PlainValue.defaultTags.SEQ;
        case PlainValue.Type.PLAIN:
          return nonSpecific ? PlainValue.defaultTags.STR : null;
        default:
          return null;
      }
    }
    function resolveByTagName(doc, node, tagName) {
      const {
        tags
      } = doc.schema;
      const matchWithTest = [];
      for (const tag of tags) {
        if (tag.tag === tagName) {
          if (tag.test) matchWithTest.push(tag);
          else {
            const res = tag.resolve(doc, node);
            return res instanceof Collection ? res : new Scalar(res);
          }
        }
      }
      const str = resolveString(doc, node);
      if (typeof str === "string" && matchWithTest.length > 0) return resolveScalar(str, matchWithTest, tags.scalarFallback);
      return null;
    }
    function getFallbackTagName({
      type
    }) {
      switch (type) {
        case PlainValue.Type.FLOW_MAP:
        case PlainValue.Type.MAP:
          return PlainValue.defaultTags.MAP;
        case PlainValue.Type.FLOW_SEQ:
        case PlainValue.Type.SEQ:
          return PlainValue.defaultTags.SEQ;
        default:
          return PlainValue.defaultTags.STR;
      }
    }
    function resolveTag(doc, node, tagName) {
      try {
        const res = resolveByTagName(doc, node, tagName);
        if (res) {
          if (tagName && node.tag) res.tag = tagName;
          return res;
        }
      } catch (error) {
        if (!error.source) error.source = node;
        doc.errors.push(error);
        return null;
      }
      try {
        const fallback = getFallbackTagName(node);
        if (!fallback) throw new Error(`The tag ${tagName} is unavailable`);
        const msg = `The tag ${tagName} is unavailable, falling back to ${fallback}`;
        doc.warnings.push(new PlainValue.YAMLWarning(node, msg));
        const res = resolveByTagName(doc, node, fallback);
        res.tag = tagName;
        return res;
      } catch (error) {
        const refError = new PlainValue.YAMLReferenceError(node, error.message);
        refError.stack = error.stack;
        doc.errors.push(refError);
        return null;
      }
    }
    var isCollectionItem = (node) => {
      if (!node) return false;
      const {
        type
      } = node;
      return type === PlainValue.Type.MAP_KEY || type === PlainValue.Type.MAP_VALUE || type === PlainValue.Type.SEQ_ITEM;
    };
    function resolveNodeProps(errors, node) {
      const comments = {
        before: [],
        after: []
      };
      let hasAnchor = false;
      let hasTag = false;
      const props = isCollectionItem(node.context.parent) ? node.context.parent.props.concat(node.props) : node.props;
      for (const {
        start,
        end
      } of props) {
        switch (node.context.src[start]) {
          case PlainValue.Char.COMMENT: {
            if (!node.commentHasRequiredWhitespace(start)) {
              const msg = "Comments must be separated from other tokens by white space characters";
              errors.push(new PlainValue.YAMLSemanticError(node, msg));
            }
            const {
              header,
              valueRange
            } = node;
            const cc = valueRange && (start > valueRange.start || header && start > header.start) ? comments.after : comments.before;
            cc.push(node.context.src.slice(start + 1, end));
            break;
          }
          // Actual anchor & tag resolution is handled by schema, here we just complain
          case PlainValue.Char.ANCHOR:
            if (hasAnchor) {
              const msg = "A node can have at most one anchor";
              errors.push(new PlainValue.YAMLSemanticError(node, msg));
            }
            hasAnchor = true;
            break;
          case PlainValue.Char.TAG:
            if (hasTag) {
              const msg = "A node can have at most one tag";
              errors.push(new PlainValue.YAMLSemanticError(node, msg));
            }
            hasTag = true;
            break;
        }
      }
      return {
        comments,
        hasAnchor,
        hasTag
      };
    }
    function resolveNodeValue(doc, node) {
      const {
        anchors,
        errors,
        schema
      } = doc;
      if (node.type === PlainValue.Type.ALIAS) {
        const name = node.rawValue;
        const src = anchors.getNode(name);
        if (!src) {
          const msg = `Aliased anchor not found: ${name}`;
          errors.push(new PlainValue.YAMLReferenceError(node, msg));
          return null;
        }
        const res = new Alias(src);
        anchors._cstAliases.push(res);
        return res;
      }
      const tagName = resolveTagName(doc, node);
      if (tagName) return resolveTag(doc, node, tagName);
      if (node.type !== PlainValue.Type.PLAIN) {
        const msg = `Failed to resolve ${node.type} node here`;
        errors.push(new PlainValue.YAMLSyntaxError(node, msg));
        return null;
      }
      try {
        const str = resolveString(doc, node);
        return resolveScalar(str, schema.tags, schema.tags.scalarFallback);
      } catch (error) {
        if (!error.source) error.source = node;
        errors.push(error);
        return null;
      }
    }
    function resolveNode(doc, node) {
      if (!node) return null;
      if (node.error) doc.errors.push(node.error);
      const {
        comments,
        hasAnchor,
        hasTag
      } = resolveNodeProps(doc.errors, node);
      if (hasAnchor) {
        const {
          anchors
        } = doc;
        const name = node.anchor;
        const prev = anchors.getNode(name);
        if (prev) anchors.map[anchors.newName(name)] = prev;
        anchors.map[name] = node;
      }
      if (node.type === PlainValue.Type.ALIAS && (hasAnchor || hasTag)) {
        const msg = "An alias node must not specify any properties";
        doc.errors.push(new PlainValue.YAMLSemanticError(node, msg));
      }
      const res = resolveNodeValue(doc, node);
      if (res) {
        res.range = [node.range.start, node.range.end];
        if (doc.options.keepCstNodes) res.cstNode = node;
        if (doc.options.keepNodeTypes) res.type = node.type;
        const cb = comments.before.join("\n");
        if (cb) {
          res.commentBefore = res.commentBefore ? `${res.commentBefore}
${cb}` : cb;
        }
        const ca = comments.after.join("\n");
        if (ca) res.comment = res.comment ? `${res.comment}
${ca}` : ca;
      }
      return node.resolved = res;
    }
    function resolveMap(doc, cst) {
      if (cst.type !== PlainValue.Type.MAP && cst.type !== PlainValue.Type.FLOW_MAP) {
        const msg = `A ${cst.type} node cannot be resolved as a mapping`;
        doc.errors.push(new PlainValue.YAMLSyntaxError(cst, msg));
        return null;
      }
      const {
        comments,
        items
      } = cst.type === PlainValue.Type.FLOW_MAP ? resolveFlowMapItems(doc, cst) : resolveBlockMapItems(doc, cst);
      const map = new YAMLMap();
      map.items = items;
      resolveComments(map, comments);
      let hasCollectionKey = false;
      for (let i = 0; i < items.length; ++i) {
        const {
          key: iKey
        } = items[i];
        if (iKey instanceof Collection) hasCollectionKey = true;
        if (doc.schema.merge && iKey && iKey.value === MERGE_KEY) {
          items[i] = new Merge(items[i]);
          const sources = items[i].value.items;
          let error = null;
          sources.some((node) => {
            if (node instanceof Alias) {
              const {
                type
              } = node.source;
              if (type === PlainValue.Type.MAP || type === PlainValue.Type.FLOW_MAP) return false;
              return error = "Merge nodes aliases can only point to maps";
            }
            return error = "Merge nodes can only have Alias nodes as values";
          });
          if (error) doc.errors.push(new PlainValue.YAMLSemanticError(cst, error));
        } else {
          for (let j = i + 1; j < items.length; ++j) {
            const {
              key: jKey
            } = items[j];
            if (iKey === jKey || iKey && jKey && Object.prototype.hasOwnProperty.call(iKey, "value") && iKey.value === jKey.value) {
              const msg = `Map keys must be unique; "${iKey}" is repeated`;
              doc.errors.push(new PlainValue.YAMLSemanticError(cst, msg));
              break;
            }
          }
        }
      }
      if (hasCollectionKey && !doc.options.mapAsMap) {
        const warn = "Keys with collection values will be stringified as YAML due to JS Object restrictions. Use mapAsMap: true to avoid this.";
        doc.warnings.push(new PlainValue.YAMLWarning(cst, warn));
      }
      cst.resolved = map;
      return map;
    }
    var valueHasPairComment = ({
      context: {
        lineStart,
        node,
        src
      },
      props
    }) => {
      if (props.length === 0) return false;
      const {
        start
      } = props[0];
      if (node && start > node.valueRange.start) return false;
      if (src[start] !== PlainValue.Char.COMMENT) return false;
      for (let i = lineStart; i < start; ++i) if (src[i] === "\n") return false;
      return true;
    };
    function resolvePairComment(item, pair) {
      if (!valueHasPairComment(item)) return;
      const comment = item.getPropValue(0, PlainValue.Char.COMMENT, true);
      let found = false;
      const cb = pair.value.commentBefore;
      if (cb && cb.startsWith(comment)) {
        pair.value.commentBefore = cb.substr(comment.length + 1);
        found = true;
      } else {
        const cc = pair.value.comment;
        if (!item.node && cc && cc.startsWith(comment)) {
          pair.value.comment = cc.substr(comment.length + 1);
          found = true;
        }
      }
      if (found) pair.comment = comment;
    }
    function resolveBlockMapItems(doc, cst) {
      const comments = [];
      const items = [];
      let key = void 0;
      let keyStart = null;
      for (let i = 0; i < cst.items.length; ++i) {
        const item = cst.items[i];
        switch (item.type) {
          case PlainValue.Type.BLANK_LINE:
            comments.push({
              afterKey: !!key,
              before: items.length
            });
            break;
          case PlainValue.Type.COMMENT:
            comments.push({
              afterKey: !!key,
              before: items.length,
              comment: item.comment
            });
            break;
          case PlainValue.Type.MAP_KEY:
            if (key !== void 0) items.push(new Pair(key));
            if (item.error) doc.errors.push(item.error);
            key = resolveNode(doc, item.node);
            keyStart = null;
            break;
          case PlainValue.Type.MAP_VALUE:
            {
              if (key === void 0) key = null;
              if (item.error) doc.errors.push(item.error);
              if (!item.context.atLineStart && item.node && item.node.type === PlainValue.Type.MAP && !item.node.context.atLineStart) {
                const msg = "Nested mappings are not allowed in compact mappings";
                doc.errors.push(new PlainValue.YAMLSemanticError(item.node, msg));
              }
              let valueNode = item.node;
              if (!valueNode && item.props.length > 0) {
                valueNode = new PlainValue.PlainValue(PlainValue.Type.PLAIN, []);
                valueNode.context = {
                  parent: item,
                  src: item.context.src
                };
                const pos = item.range.start + 1;
                valueNode.range = {
                  start: pos,
                  end: pos
                };
                valueNode.valueRange = {
                  start: pos,
                  end: pos
                };
                if (typeof item.range.origStart === "number") {
                  const origPos = item.range.origStart + 1;
                  valueNode.range.origStart = valueNode.range.origEnd = origPos;
                  valueNode.valueRange.origStart = valueNode.valueRange.origEnd = origPos;
                }
              }
              const pair = new Pair(key, resolveNode(doc, valueNode));
              resolvePairComment(item, pair);
              items.push(pair);
              if (key && typeof keyStart === "number") {
                if (item.range.start > keyStart + 1024) doc.errors.push(getLongKeyError(cst, key));
              }
              key = void 0;
              keyStart = null;
            }
            break;
          default:
            if (key !== void 0) items.push(new Pair(key));
            key = resolveNode(doc, item);
            keyStart = item.range.start;
            if (item.error) doc.errors.push(item.error);
            next: for (let j = i + 1; ; ++j) {
              const nextItem = cst.items[j];
              switch (nextItem && nextItem.type) {
                case PlainValue.Type.BLANK_LINE:
                case PlainValue.Type.COMMENT:
                  continue next;
                case PlainValue.Type.MAP_VALUE:
                  break next;
                default: {
                  const msg = "Implicit map keys need to be followed by map values";
                  doc.errors.push(new PlainValue.YAMLSemanticError(item, msg));
                  break next;
                }
              }
            }
            if (item.valueRangeContainsNewline) {
              const msg = "Implicit map keys need to be on a single line";
              doc.errors.push(new PlainValue.YAMLSemanticError(item, msg));
            }
        }
      }
      if (key !== void 0) items.push(new Pair(key));
      return {
        comments,
        items
      };
    }
    function resolveFlowMapItems(doc, cst) {
      const comments = [];
      const items = [];
      let key = void 0;
      let explicitKey = false;
      let next = "{";
      for (let i = 0; i < cst.items.length; ++i) {
        const item = cst.items[i];
        if (typeof item.char === "string") {
          const {
            char,
            offset
          } = item;
          if (char === "?" && key === void 0 && !explicitKey) {
            explicitKey = true;
            next = ":";
            continue;
          }
          if (char === ":") {
            if (key === void 0) key = null;
            if (next === ":") {
              next = ",";
              continue;
            }
          } else {
            if (explicitKey) {
              if (key === void 0 && char !== ",") key = null;
              explicitKey = false;
            }
            if (key !== void 0) {
              items.push(new Pair(key));
              key = void 0;
              if (char === ",") {
                next = ":";
                continue;
              }
            }
          }
          if (char === "}") {
            if (i === cst.items.length - 1) continue;
          } else if (char === next) {
            next = ":";
            continue;
          }
          const msg = `Flow map contains an unexpected ${char}`;
          const err = new PlainValue.YAMLSyntaxError(cst, msg);
          err.offset = offset;
          doc.errors.push(err);
        } else if (item.type === PlainValue.Type.BLANK_LINE) {
          comments.push({
            afterKey: !!key,
            before: items.length
          });
        } else if (item.type === PlainValue.Type.COMMENT) {
          checkFlowCommentSpace(doc.errors, item);
          comments.push({
            afterKey: !!key,
            before: items.length,
            comment: item.comment
          });
        } else if (key === void 0) {
          if (next === ",") doc.errors.push(new PlainValue.YAMLSemanticError(item, "Separator , missing in flow map"));
          key = resolveNode(doc, item);
        } else {
          if (next !== ",") doc.errors.push(new PlainValue.YAMLSemanticError(item, "Indicator : missing in flow map entry"));
          items.push(new Pair(key, resolveNode(doc, item)));
          key = void 0;
          explicitKey = false;
        }
      }
      checkFlowCollectionEnd(doc.errors, cst);
      if (key !== void 0) items.push(new Pair(key));
      return {
        comments,
        items
      };
    }
    function resolveSeq(doc, cst) {
      if (cst.type !== PlainValue.Type.SEQ && cst.type !== PlainValue.Type.FLOW_SEQ) {
        const msg = `A ${cst.type} node cannot be resolved as a sequence`;
        doc.errors.push(new PlainValue.YAMLSyntaxError(cst, msg));
        return null;
      }
      const {
        comments,
        items
      } = cst.type === PlainValue.Type.FLOW_SEQ ? resolveFlowSeqItems(doc, cst) : resolveBlockSeqItems(doc, cst);
      const seq = new YAMLSeq();
      seq.items = items;
      resolveComments(seq, comments);
      if (!doc.options.mapAsMap && items.some((it) => it instanceof Pair && it.key instanceof Collection)) {
        const warn = "Keys with collection values will be stringified as YAML due to JS Object restrictions. Use mapAsMap: true to avoid this.";
        doc.warnings.push(new PlainValue.YAMLWarning(cst, warn));
      }
      cst.resolved = seq;
      return seq;
    }
    function resolveBlockSeqItems(doc, cst) {
      const comments = [];
      const items = [];
      for (let i = 0; i < cst.items.length; ++i) {
        const item = cst.items[i];
        switch (item.type) {
          case PlainValue.Type.BLANK_LINE:
            comments.push({
              before: items.length
            });
            break;
          case PlainValue.Type.COMMENT:
            comments.push({
              comment: item.comment,
              before: items.length
            });
            break;
          case PlainValue.Type.SEQ_ITEM:
            if (item.error) doc.errors.push(item.error);
            items.push(resolveNode(doc, item.node));
            if (item.hasProps) {
              const msg = "Sequence items cannot have tags or anchors before the - indicator";
              doc.errors.push(new PlainValue.YAMLSemanticError(item, msg));
            }
            break;
          default:
            if (item.error) doc.errors.push(item.error);
            doc.errors.push(new PlainValue.YAMLSyntaxError(item, `Unexpected ${item.type} node in sequence`));
        }
      }
      return {
        comments,
        items
      };
    }
    function resolveFlowSeqItems(doc, cst) {
      const comments = [];
      const items = [];
      let explicitKey = false;
      let key = void 0;
      let keyStart = null;
      let next = "[";
      let prevItem = null;
      for (let i = 0; i < cst.items.length; ++i) {
        const item = cst.items[i];
        if (typeof item.char === "string") {
          const {
            char,
            offset
          } = item;
          if (char !== ":" && (explicitKey || key !== void 0)) {
            if (explicitKey && key === void 0) key = next ? items.pop() : null;
            items.push(new Pair(key));
            explicitKey = false;
            key = void 0;
            keyStart = null;
          }
          if (char === next) {
            next = null;
          } else if (!next && char === "?") {
            explicitKey = true;
          } else if (next !== "[" && char === ":" && key === void 0) {
            if (next === ",") {
              key = items.pop();
              if (key instanceof Pair) {
                const msg = "Chaining flow sequence pairs is invalid";
                const err = new PlainValue.YAMLSemanticError(cst, msg);
                err.offset = offset;
                doc.errors.push(err);
              }
              if (!explicitKey && typeof keyStart === "number") {
                const keyEnd = item.range ? item.range.start : item.offset;
                if (keyEnd > keyStart + 1024) doc.errors.push(getLongKeyError(cst, key));
                const {
                  src
                } = prevItem.context;
                for (let i2 = keyStart; i2 < keyEnd; ++i2) if (src[i2] === "\n") {
                  const msg = "Implicit keys of flow sequence pairs need to be on a single line";
                  doc.errors.push(new PlainValue.YAMLSemanticError(prevItem, msg));
                  break;
                }
              }
            } else {
              key = null;
            }
            keyStart = null;
            explicitKey = false;
            next = null;
          } else if (next === "[" || char !== "]" || i < cst.items.length - 1) {
            const msg = `Flow sequence contains an unexpected ${char}`;
            const err = new PlainValue.YAMLSyntaxError(cst, msg);
            err.offset = offset;
            doc.errors.push(err);
          }
        } else if (item.type === PlainValue.Type.BLANK_LINE) {
          comments.push({
            before: items.length
          });
        } else if (item.type === PlainValue.Type.COMMENT) {
          checkFlowCommentSpace(doc.errors, item);
          comments.push({
            comment: item.comment,
            before: items.length
          });
        } else {
          if (next) {
            const msg = `Expected a ${next} in flow sequence`;
            doc.errors.push(new PlainValue.YAMLSemanticError(item, msg));
          }
          const value = resolveNode(doc, item);
          if (key === void 0) {
            items.push(value);
            prevItem = item;
          } else {
            items.push(new Pair(key, value));
            key = void 0;
          }
          keyStart = item.range.start;
          next = ",";
        }
      }
      checkFlowCollectionEnd(doc.errors, cst);
      if (key !== void 0) items.push(new Pair(key));
      return {
        comments,
        items
      };
    }
    exports2.Alias = Alias;
    exports2.Collection = Collection;
    exports2.Merge = Merge;
    exports2.Node = Node;
    exports2.Pair = Pair;
    exports2.Scalar = Scalar;
    exports2.YAMLMap = YAMLMap;
    exports2.YAMLSeq = YAMLSeq;
    exports2.addComment = addComment;
    exports2.binaryOptions = binaryOptions;
    exports2.boolOptions = boolOptions;
    exports2.findPair = findPair;
    exports2.intOptions = intOptions;
    exports2.isEmptyPath = isEmptyPath;
    exports2.nullOptions = nullOptions;
    exports2.resolveMap = resolveMap;
    exports2.resolveNode = resolveNode;
    exports2.resolveSeq = resolveSeq;
    exports2.resolveString = resolveString;
    exports2.strOptions = strOptions;
    exports2.stringifyNumber = stringifyNumber;
    exports2.stringifyString = stringifyString;
    exports2.toJSON = toJSON;
  }
});

// node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/warnings-1000a372.js
var require_warnings_1000a372 = __commonJS({
  "node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/warnings-1000a372.js"(exports2) {
    "use strict";
    var PlainValue = require_PlainValue_ec8e588e();
    var resolveSeq = require_resolveSeq_d03cb037();
    var binary = {
      identify: (value) => value instanceof Uint8Array,
      // Buffer inherits from Uint8Array
      default: false,
      tag: "tag:yaml.org,2002:binary",
      /**
       * Returns a Buffer in node and an Uint8Array in browsers
       *
       * To use the resulting buffer as an image, you'll want to do something like:
       *
       *   const blob = new Blob([buffer], { type: 'image/jpeg' })
       *   document.querySelector('#photo').src = URL.createObjectURL(blob)
       */
      resolve: (doc, node) => {
        const src = resolveSeq.resolveString(doc, node);
        if (typeof Buffer === "function") {
          return Buffer.from(src, "base64");
        } else if (typeof atob === "function") {
          const str = atob(src.replace(/[\n\r]/g, ""));
          const buffer = new Uint8Array(str.length);
          for (let i = 0; i < str.length; ++i) buffer[i] = str.charCodeAt(i);
          return buffer;
        } else {
          const msg = "This environment does not support reading binary tags; either Buffer or atob is required";
          doc.errors.push(new PlainValue.YAMLReferenceError(node, msg));
          return null;
        }
      },
      options: resolveSeq.binaryOptions,
      stringify: ({
        comment,
        type,
        value
      }, ctx, onComment, onChompKeep) => {
        let src;
        if (typeof Buffer === "function") {
          src = value instanceof Buffer ? value.toString("base64") : Buffer.from(value.buffer).toString("base64");
        } else if (typeof btoa === "function") {
          let s = "";
          for (let i = 0; i < value.length; ++i) s += String.fromCharCode(value[i]);
          src = btoa(s);
        } else {
          throw new Error("This environment does not support writing binary tags; either Buffer or btoa is required");
        }
        if (!type) type = resolveSeq.binaryOptions.defaultType;
        if (type === PlainValue.Type.QUOTE_DOUBLE) {
          value = src;
        } else {
          const {
            lineWidth
          } = resolveSeq.binaryOptions;
          const n = Math.ceil(src.length / lineWidth);
          const lines = new Array(n);
          for (let i = 0, o = 0; i < n; ++i, o += lineWidth) {
            lines[i] = src.substr(o, lineWidth);
          }
          value = lines.join(type === PlainValue.Type.BLOCK_LITERAL ? "\n" : " ");
        }
        return resolveSeq.stringifyString({
          comment,
          type,
          value
        }, ctx, onComment, onChompKeep);
      }
    };
    function parsePairs(doc, cst) {
      const seq = resolveSeq.resolveSeq(doc, cst);
      for (let i = 0; i < seq.items.length; ++i) {
        let item = seq.items[i];
        if (item instanceof resolveSeq.Pair) continue;
        else if (item instanceof resolveSeq.YAMLMap) {
          if (item.items.length > 1) {
            const msg = "Each pair must have its own sequence indicator";
            throw new PlainValue.YAMLSemanticError(cst, msg);
          }
          const pair = item.items[0] || new resolveSeq.Pair();
          if (item.commentBefore) pair.commentBefore = pair.commentBefore ? `${item.commentBefore}
${pair.commentBefore}` : item.commentBefore;
          if (item.comment) pair.comment = pair.comment ? `${item.comment}
${pair.comment}` : item.comment;
          item = pair;
        }
        seq.items[i] = item instanceof resolveSeq.Pair ? item : new resolveSeq.Pair(item);
      }
      return seq;
    }
    function createPairs(schema, iterable, ctx) {
      const pairs2 = new resolveSeq.YAMLSeq(schema);
      pairs2.tag = "tag:yaml.org,2002:pairs";
      for (const it of iterable) {
        let key, value;
        if (Array.isArray(it)) {
          if (it.length === 2) {
            key = it[0];
            value = it[1];
          } else throw new TypeError(`Expected [key, value] tuple: ${it}`);
        } else if (it && it instanceof Object) {
          const keys = Object.keys(it);
          if (keys.length === 1) {
            key = keys[0];
            value = it[key];
          } else throw new TypeError(`Expected { key: value } tuple: ${it}`);
        } else {
          key = it;
        }
        const pair = schema.createPair(key, value, ctx);
        pairs2.items.push(pair);
      }
      return pairs2;
    }
    var pairs = {
      default: false,
      tag: "tag:yaml.org,2002:pairs",
      resolve: parsePairs,
      createNode: createPairs
    };
    var YAMLOMap = class _YAMLOMap extends resolveSeq.YAMLSeq {
      constructor() {
        super();
        PlainValue._defineProperty(this, "add", resolveSeq.YAMLMap.prototype.add.bind(this));
        PlainValue._defineProperty(this, "delete", resolveSeq.YAMLMap.prototype.delete.bind(this));
        PlainValue._defineProperty(this, "get", resolveSeq.YAMLMap.prototype.get.bind(this));
        PlainValue._defineProperty(this, "has", resolveSeq.YAMLMap.prototype.has.bind(this));
        PlainValue._defineProperty(this, "set", resolveSeq.YAMLMap.prototype.set.bind(this));
        this.tag = _YAMLOMap.tag;
      }
      toJSON(_, ctx) {
        const map = /* @__PURE__ */ new Map();
        if (ctx && ctx.onCreate) ctx.onCreate(map);
        for (const pair of this.items) {
          let key, value;
          if (pair instanceof resolveSeq.Pair) {
            key = resolveSeq.toJSON(pair.key, "", ctx);
            value = resolveSeq.toJSON(pair.value, key, ctx);
          } else {
            key = resolveSeq.toJSON(pair, "", ctx);
          }
          if (map.has(key)) throw new Error("Ordered maps must not include duplicate keys");
          map.set(key, value);
        }
        return map;
      }
    };
    PlainValue._defineProperty(YAMLOMap, "tag", "tag:yaml.org,2002:omap");
    function parseOMap(doc, cst) {
      const pairs2 = parsePairs(doc, cst);
      const seenKeys = [];
      for (const {
        key
      } of pairs2.items) {
        if (key instanceof resolveSeq.Scalar) {
          if (seenKeys.includes(key.value)) {
            const msg = "Ordered maps must not include duplicate keys";
            throw new PlainValue.YAMLSemanticError(cst, msg);
          } else {
            seenKeys.push(key.value);
          }
        }
      }
      return Object.assign(new YAMLOMap(), pairs2);
    }
    function createOMap(schema, iterable, ctx) {
      const pairs2 = createPairs(schema, iterable, ctx);
      const omap2 = new YAMLOMap();
      omap2.items = pairs2.items;
      return omap2;
    }
    var omap = {
      identify: (value) => value instanceof Map,
      nodeClass: YAMLOMap,
      default: false,
      tag: "tag:yaml.org,2002:omap",
      resolve: parseOMap,
      createNode: createOMap
    };
    var YAMLSet = class _YAMLSet extends resolveSeq.YAMLMap {
      constructor() {
        super();
        this.tag = _YAMLSet.tag;
      }
      add(key) {
        const pair = key instanceof resolveSeq.Pair ? key : new resolveSeq.Pair(key);
        const prev = resolveSeq.findPair(this.items, pair.key);
        if (!prev) this.items.push(pair);
      }
      get(key, keepPair) {
        const pair = resolveSeq.findPair(this.items, key);
        return !keepPair && pair instanceof resolveSeq.Pair ? pair.key instanceof resolveSeq.Scalar ? pair.key.value : pair.key : pair;
      }
      set(key, value) {
        if (typeof value !== "boolean") throw new Error(`Expected boolean value for set(key, value) in a YAML set, not ${typeof value}`);
        const prev = resolveSeq.findPair(this.items, key);
        if (prev && !value) {
          this.items.splice(this.items.indexOf(prev), 1);
        } else if (!prev && value) {
          this.items.push(new resolveSeq.Pair(key));
        }
      }
      toJSON(_, ctx) {
        return super.toJSON(_, ctx, Set);
      }
      toString(ctx, onComment, onChompKeep) {
        if (!ctx) return JSON.stringify(this);
        if (this.hasAllNullValues()) return super.toString(ctx, onComment, onChompKeep);
        else throw new Error("Set items must all have null values");
      }
    };
    PlainValue._defineProperty(YAMLSet, "tag", "tag:yaml.org,2002:set");
    function parseSet(doc, cst) {
      const map = resolveSeq.resolveMap(doc, cst);
      if (!map.hasAllNullValues()) throw new PlainValue.YAMLSemanticError(cst, "Set items must all have null values");
      return Object.assign(new YAMLSet(), map);
    }
    function createSet(schema, iterable, ctx) {
      const set2 = new YAMLSet();
      for (const value of iterable) set2.items.push(schema.createPair(value, null, ctx));
      return set2;
    }
    var set = {
      identify: (value) => value instanceof Set,
      nodeClass: YAMLSet,
      default: false,
      tag: "tag:yaml.org,2002:set",
      resolve: parseSet,
      createNode: createSet
    };
    var parseSexagesimal = (sign, parts) => {
      const n = parts.split(":").reduce((n2, p) => n2 * 60 + Number(p), 0);
      return sign === "-" ? -n : n;
    };
    var stringifySexagesimal = ({
      value
    }) => {
      if (isNaN(value) || !isFinite(value)) return resolveSeq.stringifyNumber(value);
      let sign = "";
      if (value < 0) {
        sign = "-";
        value = Math.abs(value);
      }
      const parts = [value % 60];
      if (value < 60) {
        parts.unshift(0);
      } else {
        value = Math.round((value - parts[0]) / 60);
        parts.unshift(value % 60);
        if (value >= 60) {
          value = Math.round((value - parts[0]) / 60);
          parts.unshift(value);
        }
      }
      return sign + parts.map((n) => n < 10 ? "0" + String(n) : String(n)).join(":").replace(/000000\d*$/, "");
    };
    var intTime = {
      identify: (value) => typeof value === "number",
      default: true,
      tag: "tag:yaml.org,2002:int",
      format: "TIME",
      test: /^([-+]?)([0-9][0-9_]*(?::[0-5]?[0-9])+)$/,
      resolve: (str, sign, parts) => parseSexagesimal(sign, parts.replace(/_/g, "")),
      stringify: stringifySexagesimal
    };
    var floatTime = {
      identify: (value) => typeof value === "number",
      default: true,
      tag: "tag:yaml.org,2002:float",
      format: "TIME",
      test: /^([-+]?)([0-9][0-9_]*(?::[0-5]?[0-9])+\.[0-9_]*)$/,
      resolve: (str, sign, parts) => parseSexagesimal(sign, parts.replace(/_/g, "")),
      stringify: stringifySexagesimal
    };
    var timestamp = {
      identify: (value) => value instanceof Date,
      default: true,
      tag: "tag:yaml.org,2002:timestamp",
      // If the time zone is omitted, the timestamp is assumed to be specified in UTC. The time part
      // may be omitted altogether, resulting in a date format. In such a case, the time part is
      // assumed to be 00:00:00Z (start of day, UTC).
      test: RegExp("^(?:([0-9]{4})-([0-9]{1,2})-([0-9]{1,2})(?:(?:t|T|[ \\t]+)([0-9]{1,2}):([0-9]{1,2}):([0-9]{1,2}(\\.[0-9]+)?)(?:[ \\t]*(Z|[-+][012]?[0-9](?::[0-9]{2})?))?)?)$"),
      resolve: (str, year, month, day, hour, minute, second, millisec, tz) => {
        if (millisec) millisec = (millisec + "00").substr(1, 3);
        let date = Date.UTC(year, month - 1, day, hour || 0, minute || 0, second || 0, millisec || 0);
        if (tz && tz !== "Z") {
          let d = parseSexagesimal(tz[0], tz.slice(1));
          if (Math.abs(d) < 30) d *= 60;
          date -= 6e4 * d;
        }
        return new Date(date);
      },
      stringify: ({
        value
      }) => value.toISOString().replace(/((T00:00)?:00)?\.000Z$/, "")
    };
    function shouldWarn(deprecation) {
      const env = typeof process !== "undefined" && process.env || {};
      if (deprecation) {
        if (typeof YAML_SILENCE_DEPRECATION_WARNINGS !== "undefined") return !YAML_SILENCE_DEPRECATION_WARNINGS;
        return !env.YAML_SILENCE_DEPRECATION_WARNINGS;
      }
      if (typeof YAML_SILENCE_WARNINGS !== "undefined") return !YAML_SILENCE_WARNINGS;
      return !env.YAML_SILENCE_WARNINGS;
    }
    function warn(warning, type) {
      if (shouldWarn(false)) {
        const emit = typeof process !== "undefined" && process.emitWarning;
        if (emit) emit(warning, type);
        else {
          console.warn(type ? `${type}: ${warning}` : warning);
        }
      }
    }
    function warnFileDeprecation(filename) {
      if (shouldWarn(true)) {
        const path = filename.replace(/.*yaml[/\\]/i, "").replace(/\.js$/, "").replace(/\\/g, "/");
        warn(`The endpoint 'yaml/${path}' will be removed in a future release.`, "DeprecationWarning");
      }
    }
    var warned = {};
    function warnOptionDeprecation(name, alternative) {
      if (!warned[name] && shouldWarn(true)) {
        warned[name] = true;
        let msg = `The option '${name}' will be removed in a future release`;
        msg += alternative ? `, use '${alternative}' instead.` : ".";
        warn(msg, "DeprecationWarning");
      }
    }
    exports2.binary = binary;
    exports2.floatTime = floatTime;
    exports2.intTime = intTime;
    exports2.omap = omap;
    exports2.pairs = pairs;
    exports2.set = set;
    exports2.timestamp = timestamp;
    exports2.warn = warn;
    exports2.warnFileDeprecation = warnFileDeprecation;
    exports2.warnOptionDeprecation = warnOptionDeprecation;
  }
});

// node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/Schema-88e323a7.js
var require_Schema_88e323a7 = __commonJS({
  "node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/Schema-88e323a7.js"(exports2) {
    "use strict";
    var PlainValue = require_PlainValue_ec8e588e();
    var resolveSeq = require_resolveSeq_d03cb037();
    var warnings = require_warnings_1000a372();
    function createMap(schema, obj, ctx) {
      const map2 = new resolveSeq.YAMLMap(schema);
      if (obj instanceof Map) {
        for (const [key, value] of obj) map2.items.push(schema.createPair(key, value, ctx));
      } else if (obj && typeof obj === "object") {
        for (const key of Object.keys(obj)) map2.items.push(schema.createPair(key, obj[key], ctx));
      }
      if (typeof schema.sortMapEntries === "function") {
        map2.items.sort(schema.sortMapEntries);
      }
      return map2;
    }
    var map = {
      createNode: createMap,
      default: true,
      nodeClass: resolveSeq.YAMLMap,
      tag: "tag:yaml.org,2002:map",
      resolve: resolveSeq.resolveMap
    };
    function createSeq(schema, obj, ctx) {
      const seq2 = new resolveSeq.YAMLSeq(schema);
      if (obj && obj[Symbol.iterator]) {
        for (const it of obj) {
          const v = schema.createNode(it, ctx.wrapScalars, null, ctx);
          seq2.items.push(v);
        }
      }
      return seq2;
    }
    var seq = {
      createNode: createSeq,
      default: true,
      nodeClass: resolveSeq.YAMLSeq,
      tag: "tag:yaml.org,2002:seq",
      resolve: resolveSeq.resolveSeq
    };
    var string = {
      identify: (value) => typeof value === "string",
      default: true,
      tag: "tag:yaml.org,2002:str",
      resolve: resolveSeq.resolveString,
      stringify(item, ctx, onComment, onChompKeep) {
        ctx = Object.assign({
          actualString: true
        }, ctx);
        return resolveSeq.stringifyString(item, ctx, onComment, onChompKeep);
      },
      options: resolveSeq.strOptions
    };
    var failsafe = [map, seq, string];
    var intIdentify$2 = (value) => typeof value === "bigint" || Number.isInteger(value);
    var intResolve$1 = (src, part, radix) => resolveSeq.intOptions.asBigInt ? BigInt(src) : parseInt(part, radix);
    function intStringify$1(node, radix, prefix) {
      const {
        value
      } = node;
      if (intIdentify$2(value) && value >= 0) return prefix + value.toString(radix);
      return resolveSeq.stringifyNumber(node);
    }
    var nullObj = {
      identify: (value) => value == null,
      createNode: (schema, value, ctx) => ctx.wrapScalars ? new resolveSeq.Scalar(null) : null,
      default: true,
      tag: "tag:yaml.org,2002:null",
      test: /^(?:~|[Nn]ull|NULL)?$/,
      resolve: () => null,
      options: resolveSeq.nullOptions,
      stringify: () => resolveSeq.nullOptions.nullStr
    };
    var boolObj = {
      identify: (value) => typeof value === "boolean",
      default: true,
      tag: "tag:yaml.org,2002:bool",
      test: /^(?:[Tt]rue|TRUE|[Ff]alse|FALSE)$/,
      resolve: (str) => str[0] === "t" || str[0] === "T",
      options: resolveSeq.boolOptions,
      stringify: ({
        value
      }) => value ? resolveSeq.boolOptions.trueStr : resolveSeq.boolOptions.falseStr
    };
    var octObj = {
      identify: (value) => intIdentify$2(value) && value >= 0,
      default: true,
      tag: "tag:yaml.org,2002:int",
      format: "OCT",
      test: /^0o([0-7]+)$/,
      resolve: (str, oct) => intResolve$1(str, oct, 8),
      options: resolveSeq.intOptions,
      stringify: (node) => intStringify$1(node, 8, "0o")
    };
    var intObj = {
      identify: intIdentify$2,
      default: true,
      tag: "tag:yaml.org,2002:int",
      test: /^[-+]?[0-9]+$/,
      resolve: (str) => intResolve$1(str, str, 10),
      options: resolveSeq.intOptions,
      stringify: resolveSeq.stringifyNumber
    };
    var hexObj = {
      identify: (value) => intIdentify$2(value) && value >= 0,
      default: true,
      tag: "tag:yaml.org,2002:int",
      format: "HEX",
      test: /^0x([0-9a-fA-F]+)$/,
      resolve: (str, hex) => intResolve$1(str, hex, 16),
      options: resolveSeq.intOptions,
      stringify: (node) => intStringify$1(node, 16, "0x")
    };
    var nanObj = {
      identify: (value) => typeof value === "number",
      default: true,
      tag: "tag:yaml.org,2002:float",
      test: /^(?:[-+]?\.inf|(\.nan))$/i,
      resolve: (str, nan) => nan ? NaN : str[0] === "-" ? Number.NEGATIVE_INFINITY : Number.POSITIVE_INFINITY,
      stringify: resolveSeq.stringifyNumber
    };
    var expObj = {
      identify: (value) => typeof value === "number",
      default: true,
      tag: "tag:yaml.org,2002:float",
      format: "EXP",
      test: /^[-+]?(?:\.[0-9]+|[0-9]+(?:\.[0-9]*)?)[eE][-+]?[0-9]+$/,
      resolve: (str) => parseFloat(str),
      stringify: ({
        value
      }) => Number(value).toExponential()
    };
    var floatObj = {
      identify: (value) => typeof value === "number",
      default: true,
      tag: "tag:yaml.org,2002:float",
      test: /^[-+]?(?:\.([0-9]+)|[0-9]+\.([0-9]*))$/,
      resolve(str, frac1, frac2) {
        const frac = frac1 || frac2;
        const node = new resolveSeq.Scalar(parseFloat(str));
        if (frac && frac[frac.length - 1] === "0") node.minFractionDigits = frac.length;
        return node;
      },
      stringify: resolveSeq.stringifyNumber
    };
    var core = failsafe.concat([nullObj, boolObj, octObj, intObj, hexObj, nanObj, expObj, floatObj]);
    var intIdentify$1 = (value) => typeof value === "bigint" || Number.isInteger(value);
    var stringifyJSON = ({
      value
    }) => JSON.stringify(value);
    var json = [map, seq, {
      identify: (value) => typeof value === "string",
      default: true,
      tag: "tag:yaml.org,2002:str",
      resolve: resolveSeq.resolveString,
      stringify: stringifyJSON
    }, {
      identify: (value) => value == null,
      createNode: (schema, value, ctx) => ctx.wrapScalars ? new resolveSeq.Scalar(null) : null,
      default: true,
      tag: "tag:yaml.org,2002:null",
      test: /^null$/,
      resolve: () => null,
      stringify: stringifyJSON
    }, {
      identify: (value) => typeof value === "boolean",
      default: true,
      tag: "tag:yaml.org,2002:bool",
      test: /^true|false$/,
      resolve: (str) => str === "true",
      stringify: stringifyJSON
    }, {
      identify: intIdentify$1,
      default: true,
      tag: "tag:yaml.org,2002:int",
      test: /^-?(?:0|[1-9][0-9]*)$/,
      resolve: (str) => resolveSeq.intOptions.asBigInt ? BigInt(str) : parseInt(str, 10),
      stringify: ({
        value
      }) => intIdentify$1(value) ? value.toString() : JSON.stringify(value)
    }, {
      identify: (value) => typeof value === "number",
      default: true,
      tag: "tag:yaml.org,2002:float",
      test: /^-?(?:0|[1-9][0-9]*)(?:\.[0-9]*)?(?:[eE][-+]?[0-9]+)?$/,
      resolve: (str) => parseFloat(str),
      stringify: stringifyJSON
    }];
    json.scalarFallback = (str) => {
      throw new SyntaxError(`Unresolved plain scalar ${JSON.stringify(str)}`);
    };
    var boolStringify = ({
      value
    }) => value ? resolveSeq.boolOptions.trueStr : resolveSeq.boolOptions.falseStr;
    var intIdentify = (value) => typeof value === "bigint" || Number.isInteger(value);
    function intResolve(sign, src, radix) {
      let str = src.replace(/_/g, "");
      if (resolveSeq.intOptions.asBigInt) {
        switch (radix) {
          case 2:
            str = `0b${str}`;
            break;
          case 8:
            str = `0o${str}`;
            break;
          case 16:
            str = `0x${str}`;
            break;
        }
        const n2 = BigInt(str);
        return sign === "-" ? BigInt(-1) * n2 : n2;
      }
      const n = parseInt(str, radix);
      return sign === "-" ? -1 * n : n;
    }
    function intStringify(node, radix, prefix) {
      const {
        value
      } = node;
      if (intIdentify(value)) {
        const str = value.toString(radix);
        return value < 0 ? "-" + prefix + str.substr(1) : prefix + str;
      }
      return resolveSeq.stringifyNumber(node);
    }
    var yaml11 = failsafe.concat([{
      identify: (value) => value == null,
      createNode: (schema, value, ctx) => ctx.wrapScalars ? new resolveSeq.Scalar(null) : null,
      default: true,
      tag: "tag:yaml.org,2002:null",
      test: /^(?:~|[Nn]ull|NULL)?$/,
      resolve: () => null,
      options: resolveSeq.nullOptions,
      stringify: () => resolveSeq.nullOptions.nullStr
    }, {
      identify: (value) => typeof value === "boolean",
      default: true,
      tag: "tag:yaml.org,2002:bool",
      test: /^(?:Y|y|[Yy]es|YES|[Tt]rue|TRUE|[Oo]n|ON)$/,
      resolve: () => true,
      options: resolveSeq.boolOptions,
      stringify: boolStringify
    }, {
      identify: (value) => typeof value === "boolean",
      default: true,
      tag: "tag:yaml.org,2002:bool",
      test: /^(?:N|n|[Nn]o|NO|[Ff]alse|FALSE|[Oo]ff|OFF)$/i,
      resolve: () => false,
      options: resolveSeq.boolOptions,
      stringify: boolStringify
    }, {
      identify: intIdentify,
      default: true,
      tag: "tag:yaml.org,2002:int",
      format: "BIN",
      test: /^([-+]?)0b([0-1_]+)$/,
      resolve: (str, sign, bin) => intResolve(sign, bin, 2),
      stringify: (node) => intStringify(node, 2, "0b")
    }, {
      identify: intIdentify,
      default: true,
      tag: "tag:yaml.org,2002:int",
      format: "OCT",
      test: /^([-+]?)0([0-7_]+)$/,
      resolve: (str, sign, oct) => intResolve(sign, oct, 8),
      stringify: (node) => intStringify(node, 8, "0")
    }, {
      identify: intIdentify,
      default: true,
      tag: "tag:yaml.org,2002:int",
      test: /^([-+]?)([0-9][0-9_]*)$/,
      resolve: (str, sign, abs) => intResolve(sign, abs, 10),
      stringify: resolveSeq.stringifyNumber
    }, {
      identify: intIdentify,
      default: true,
      tag: "tag:yaml.org,2002:int",
      format: "HEX",
      test: /^([-+]?)0x([0-9a-fA-F_]+)$/,
      resolve: (str, sign, hex) => intResolve(sign, hex, 16),
      stringify: (node) => intStringify(node, 16, "0x")
    }, {
      identify: (value) => typeof value === "number",
      default: true,
      tag: "tag:yaml.org,2002:float",
      test: /^(?:[-+]?\.inf|(\.nan))$/i,
      resolve: (str, nan) => nan ? NaN : str[0] === "-" ? Number.NEGATIVE_INFINITY : Number.POSITIVE_INFINITY,
      stringify: resolveSeq.stringifyNumber
    }, {
      identify: (value) => typeof value === "number",
      default: true,
      tag: "tag:yaml.org,2002:float",
      format: "EXP",
      test: /^[-+]?([0-9][0-9_]*)?(\.[0-9_]*)?[eE][-+]?[0-9]+$/,
      resolve: (str) => parseFloat(str.replace(/_/g, "")),
      stringify: ({
        value
      }) => Number(value).toExponential()
    }, {
      identify: (value) => typeof value === "number",
      default: true,
      tag: "tag:yaml.org,2002:float",
      test: /^[-+]?(?:[0-9][0-9_]*)?\.([0-9_]*)$/,
      resolve(str, frac) {
        const node = new resolveSeq.Scalar(parseFloat(str.replace(/_/g, "")));
        if (frac) {
          const f = frac.replace(/_/g, "");
          if (f[f.length - 1] === "0") node.minFractionDigits = f.length;
        }
        return node;
      },
      stringify: resolveSeq.stringifyNumber
    }], warnings.binary, warnings.omap, warnings.pairs, warnings.set, warnings.intTime, warnings.floatTime, warnings.timestamp);
    var schemas = {
      core,
      failsafe,
      json,
      yaml11
    };
    var tags = {
      binary: warnings.binary,
      bool: boolObj,
      float: floatObj,
      floatExp: expObj,
      floatNaN: nanObj,
      floatTime: warnings.floatTime,
      int: intObj,
      intHex: hexObj,
      intOct: octObj,
      intTime: warnings.intTime,
      map,
      null: nullObj,
      omap: warnings.omap,
      pairs: warnings.pairs,
      seq,
      set: warnings.set,
      timestamp: warnings.timestamp
    };
    function findTagObject(value, tagName, tags2) {
      if (tagName) {
        const match = tags2.filter((t) => t.tag === tagName);
        const tagObj = match.find((t) => !t.format) || match[0];
        if (!tagObj) throw new Error(`Tag ${tagName} not found`);
        return tagObj;
      }
      return tags2.find((t) => (t.identify && t.identify(value) || t.class && value instanceof t.class) && !t.format);
    }
    function createNode(value, tagName, ctx) {
      if (value instanceof resolveSeq.Node) return value;
      const {
        defaultPrefix,
        onTagObj,
        prevObjects,
        schema,
        wrapScalars
      } = ctx;
      if (tagName && tagName.startsWith("!!")) tagName = defaultPrefix + tagName.slice(2);
      let tagObj = findTagObject(value, tagName, schema.tags);
      if (!tagObj) {
        if (typeof value.toJSON === "function") value = value.toJSON();
        if (!value || typeof value !== "object") return wrapScalars ? new resolveSeq.Scalar(value) : value;
        tagObj = value instanceof Map ? map : value[Symbol.iterator] ? seq : map;
      }
      if (onTagObj) {
        onTagObj(tagObj);
        delete ctx.onTagObj;
      }
      const obj = {
        value: void 0,
        node: void 0
      };
      if (value && typeof value === "object" && prevObjects) {
        const prev = prevObjects.get(value);
        if (prev) {
          const alias = new resolveSeq.Alias(prev);
          ctx.aliasNodes.push(alias);
          return alias;
        }
        obj.value = value;
        prevObjects.set(value, obj);
      }
      obj.node = tagObj.createNode ? tagObj.createNode(ctx.schema, value, ctx) : wrapScalars ? new resolveSeq.Scalar(value) : value;
      if (tagName && obj.node instanceof resolveSeq.Node) obj.node.tag = tagName;
      return obj.node;
    }
    function getSchemaTags(schemas2, knownTags, customTags, schemaId) {
      let tags2 = schemas2[schemaId.replace(/\W/g, "")];
      if (!tags2) {
        const keys = Object.keys(schemas2).map((key) => JSON.stringify(key)).join(", ");
        throw new Error(`Unknown schema "${schemaId}"; use one of ${keys}`);
      }
      if (Array.isArray(customTags)) {
        for (const tag of customTags) tags2 = tags2.concat(tag);
      } else if (typeof customTags === "function") {
        tags2 = customTags(tags2.slice());
      }
      for (let i = 0; i < tags2.length; ++i) {
        const tag = tags2[i];
        if (typeof tag === "string") {
          const tagObj = knownTags[tag];
          if (!tagObj) {
            const keys = Object.keys(knownTags).map((key) => JSON.stringify(key)).join(", ");
            throw new Error(`Unknown custom tag "${tag}"; use one of ${keys}`);
          }
          tags2[i] = tagObj;
        }
      }
      return tags2;
    }
    var sortMapEntriesByKey = (a, b) => a.key < b.key ? -1 : a.key > b.key ? 1 : 0;
    var Schema = class _Schema {
      // TODO: remove in v2
      // TODO: remove in v2
      constructor({
        customTags,
        merge,
        schema,
        sortMapEntries,
        tags: deprecatedCustomTags
      }) {
        this.merge = !!merge;
        this.name = schema;
        this.sortMapEntries = sortMapEntries === true ? sortMapEntriesByKey : sortMapEntries || null;
        if (!customTags && deprecatedCustomTags) warnings.warnOptionDeprecation("tags", "customTags");
        this.tags = getSchemaTags(schemas, tags, customTags || deprecatedCustomTags, schema);
      }
      createNode(value, wrapScalars, tagName, ctx) {
        const baseCtx = {
          defaultPrefix: _Schema.defaultPrefix,
          schema: this,
          wrapScalars
        };
        const createCtx = ctx ? Object.assign(ctx, baseCtx) : baseCtx;
        return createNode(value, tagName, createCtx);
      }
      createPair(key, value, ctx) {
        if (!ctx) ctx = {
          wrapScalars: true
        };
        const k = this.createNode(key, ctx.wrapScalars, null, ctx);
        const v = this.createNode(value, ctx.wrapScalars, null, ctx);
        return new resolveSeq.Pair(k, v);
      }
    };
    PlainValue._defineProperty(Schema, "defaultPrefix", PlainValue.defaultTagPrefix);
    PlainValue._defineProperty(Schema, "defaultTags", PlainValue.defaultTags);
    exports2.Schema = Schema;
  }
});

// node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/Document-9b4560a1.js
var require_Document_9b4560a1 = __commonJS({
  "node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/Document-9b4560a1.js"(exports2) {
    "use strict";
    var PlainValue = require_PlainValue_ec8e588e();
    var resolveSeq = require_resolveSeq_d03cb037();
    var Schema = require_Schema_88e323a7();
    var defaultOptions = {
      anchorPrefix: "a",
      customTags: null,
      indent: 2,
      indentSeq: true,
      keepCstNodes: false,
      keepNodeTypes: true,
      keepBlobsInJSON: true,
      mapAsMap: false,
      maxAliasCount: 100,
      prettyErrors: false,
      // TODO Set true in v2
      simpleKeys: false,
      version: "1.2"
    };
    var scalarOptions = {
      get binary() {
        return resolveSeq.binaryOptions;
      },
      set binary(opt) {
        Object.assign(resolveSeq.binaryOptions, opt);
      },
      get bool() {
        return resolveSeq.boolOptions;
      },
      set bool(opt) {
        Object.assign(resolveSeq.boolOptions, opt);
      },
      get int() {
        return resolveSeq.intOptions;
      },
      set int(opt) {
        Object.assign(resolveSeq.intOptions, opt);
      },
      get null() {
        return resolveSeq.nullOptions;
      },
      set null(opt) {
        Object.assign(resolveSeq.nullOptions, opt);
      },
      get str() {
        return resolveSeq.strOptions;
      },
      set str(opt) {
        Object.assign(resolveSeq.strOptions, opt);
      }
    };
    var documentOptions = {
      "1.0": {
        schema: "yaml-1.1",
        merge: true,
        tagPrefixes: [{
          handle: "!",
          prefix: PlainValue.defaultTagPrefix
        }, {
          handle: "!!",
          prefix: "tag:private.yaml.org,2002:"
        }]
      },
      1.1: {
        schema: "yaml-1.1",
        merge: true,
        tagPrefixes: [{
          handle: "!",
          prefix: "!"
        }, {
          handle: "!!",
          prefix: PlainValue.defaultTagPrefix
        }]
      },
      1.2: {
        schema: "core",
        merge: false,
        tagPrefixes: [{
          handle: "!",
          prefix: "!"
        }, {
          handle: "!!",
          prefix: PlainValue.defaultTagPrefix
        }]
      }
    };
    function stringifyTag(doc, tag) {
      if ((doc.version || doc.options.version) === "1.0") {
        const priv = tag.match(/^tag:private\.yaml\.org,2002:([^:/]+)$/);
        if (priv) return "!" + priv[1];
        const vocab = tag.match(/^tag:([a-zA-Z0-9-]+)\.yaml\.org,2002:(.*)/);
        return vocab ? `!${vocab[1]}/${vocab[2]}` : `!${tag.replace(/^tag:/, "")}`;
      }
      let p = doc.tagPrefixes.find((p2) => tag.indexOf(p2.prefix) === 0);
      if (!p) {
        const dtp = doc.getDefaults().tagPrefixes;
        p = dtp && dtp.find((p2) => tag.indexOf(p2.prefix) === 0);
      }
      if (!p) return tag[0] === "!" ? tag : `!<${tag}>`;
      const suffix = tag.substr(p.prefix.length).replace(/[!,[\]{}]/g, (ch) => ({
        "!": "%21",
        ",": "%2C",
        "[": "%5B",
        "]": "%5D",
        "{": "%7B",
        "}": "%7D"
      })[ch]);
      return p.handle + suffix;
    }
    function getTagObject(tags, item) {
      if (item instanceof resolveSeq.Alias) return resolveSeq.Alias;
      if (item.tag) {
        const match = tags.filter((t) => t.tag === item.tag);
        if (match.length > 0) return match.find((t) => t.format === item.format) || match[0];
      }
      let tagObj, obj;
      if (item instanceof resolveSeq.Scalar) {
        obj = item.value;
        const match = tags.filter((t) => t.identify && t.identify(obj) || t.class && obj instanceof t.class);
        tagObj = match.find((t) => t.format === item.format) || match.find((t) => !t.format);
      } else {
        obj = item;
        tagObj = tags.find((t) => t.nodeClass && obj instanceof t.nodeClass);
      }
      if (!tagObj) {
        const name = obj && obj.constructor ? obj.constructor.name : typeof obj;
        throw new Error(`Tag not resolved for ${name} value`);
      }
      return tagObj;
    }
    function stringifyProps(node, tagObj, {
      anchors,
      doc
    }) {
      const props = [];
      const anchor = doc.anchors.getName(node);
      if (anchor) {
        anchors[anchor] = node;
        props.push(`&${anchor}`);
      }
      if (node.tag) {
        props.push(stringifyTag(doc, node.tag));
      } else if (!tagObj.default) {
        props.push(stringifyTag(doc, tagObj.tag));
      }
      return props.join(" ");
    }
    function stringify(item, ctx, onComment, onChompKeep) {
      const {
        anchors,
        schema
      } = ctx.doc;
      let tagObj;
      if (!(item instanceof resolveSeq.Node)) {
        const createCtx = {
          aliasNodes: [],
          onTagObj: (o) => tagObj = o,
          prevObjects: /* @__PURE__ */ new Map()
        };
        item = schema.createNode(item, true, null, createCtx);
        for (const alias of createCtx.aliasNodes) {
          alias.source = alias.source.node;
          let name = anchors.getName(alias.source);
          if (!name) {
            name = anchors.newName();
            anchors.map[name] = alias.source;
          }
        }
      }
      if (item instanceof resolveSeq.Pair) return item.toString(ctx, onComment, onChompKeep);
      if (!tagObj) tagObj = getTagObject(schema.tags, item);
      const props = stringifyProps(item, tagObj, ctx);
      if (props.length > 0) ctx.indentAtStart = (ctx.indentAtStart || 0) + props.length + 1;
      const str = typeof tagObj.stringify === "function" ? tagObj.stringify(item, ctx, onComment, onChompKeep) : item instanceof resolveSeq.Scalar ? resolveSeq.stringifyString(item, ctx, onComment, onChompKeep) : item.toString(ctx, onComment, onChompKeep);
      if (!props) return str;
      return item instanceof resolveSeq.Scalar || str[0] === "{" || str[0] === "[" ? `${props} ${str}` : `${props}
${ctx.indent}${str}`;
    }
    var Anchors = class _Anchors {
      static validAnchorNode(node) {
        return node instanceof resolveSeq.Scalar || node instanceof resolveSeq.YAMLSeq || node instanceof resolveSeq.YAMLMap;
      }
      constructor(prefix) {
        PlainValue._defineProperty(this, "map", /* @__PURE__ */ Object.create(null));
        this.prefix = prefix;
      }
      createAlias(node, name) {
        this.setAnchor(node, name);
        return new resolveSeq.Alias(node);
      }
      createMergePair(...sources) {
        const merge = new resolveSeq.Merge();
        merge.value.items = sources.map((s) => {
          if (s instanceof resolveSeq.Alias) {
            if (s.source instanceof resolveSeq.YAMLMap) return s;
          } else if (s instanceof resolveSeq.YAMLMap) {
            return this.createAlias(s);
          }
          throw new Error("Merge sources must be Map nodes or their Aliases");
        });
        return merge;
      }
      getName(node) {
        const {
          map
        } = this;
        return Object.keys(map).find((a) => map[a] === node);
      }
      getNames() {
        return Object.keys(this.map);
      }
      getNode(name) {
        return this.map[name];
      }
      newName(prefix) {
        if (!prefix) prefix = this.prefix;
        const names = Object.keys(this.map);
        for (let i = 1; true; ++i) {
          const name = `${prefix}${i}`;
          if (!names.includes(name)) return name;
        }
      }
      // During parsing, map & aliases contain CST nodes
      resolveNodes() {
        const {
          map,
          _cstAliases
        } = this;
        Object.keys(map).forEach((a) => {
          map[a] = map[a].resolved;
        });
        _cstAliases.forEach((a) => {
          a.source = a.source.resolved;
        });
        delete this._cstAliases;
      }
      setAnchor(node, name) {
        if (node != null && !_Anchors.validAnchorNode(node)) {
          throw new Error("Anchors may only be set for Scalar, Seq and Map nodes");
        }
        if (name && /[\x00-\x19\s,[\]{}]/.test(name)) {
          throw new Error("Anchor names must not contain whitespace or control characters");
        }
        const {
          map
        } = this;
        const prev = node && Object.keys(map).find((a) => map[a] === node);
        if (prev) {
          if (!name) {
            return prev;
          } else if (prev !== name) {
            delete map[prev];
            map[name] = node;
          }
        } else {
          if (!name) {
            if (!node) return null;
            name = this.newName();
          }
          map[name] = node;
        }
        return name;
      }
    };
    var visit = (node, tags) => {
      if (node && typeof node === "object") {
        const {
          tag
        } = node;
        if (node instanceof resolveSeq.Collection) {
          if (tag) tags[tag] = true;
          node.items.forEach((n) => visit(n, tags));
        } else if (node instanceof resolveSeq.Pair) {
          visit(node.key, tags);
          visit(node.value, tags);
        } else if (node instanceof resolveSeq.Scalar) {
          if (tag) tags[tag] = true;
        }
      }
      return tags;
    };
    var listTagNames = (node) => Object.keys(visit(node, {}));
    function parseContents(doc, contents) {
      const comments = {
        before: [],
        after: []
      };
      let body = void 0;
      let spaceBefore = false;
      for (const node of contents) {
        if (node.valueRange) {
          if (body !== void 0) {
            const msg = "Document contains trailing content not separated by a ... or --- line";
            doc.errors.push(new PlainValue.YAMLSyntaxError(node, msg));
            break;
          }
          const res = resolveSeq.resolveNode(doc, node);
          if (spaceBefore) {
            res.spaceBefore = true;
            spaceBefore = false;
          }
          body = res;
        } else if (node.comment !== null) {
          const cc = body === void 0 ? comments.before : comments.after;
          cc.push(node.comment);
        } else if (node.type === PlainValue.Type.BLANK_LINE) {
          spaceBefore = true;
          if (body === void 0 && comments.before.length > 0 && !doc.commentBefore) {
            doc.commentBefore = comments.before.join("\n");
            comments.before = [];
          }
        }
      }
      doc.contents = body || null;
      if (!body) {
        doc.comment = comments.before.concat(comments.after).join("\n") || null;
      } else {
        const cb = comments.before.join("\n");
        if (cb) {
          const cbNode = body instanceof resolveSeq.Collection && body.items[0] ? body.items[0] : body;
          cbNode.commentBefore = cbNode.commentBefore ? `${cb}
${cbNode.commentBefore}` : cb;
        }
        doc.comment = comments.after.join("\n") || null;
      }
    }
    function resolveTagDirective({
      tagPrefixes
    }, directive) {
      const [handle, prefix] = directive.parameters;
      if (!handle || !prefix) {
        const msg = "Insufficient parameters given for %TAG directive";
        throw new PlainValue.YAMLSemanticError(directive, msg);
      }
      if (tagPrefixes.some((p) => p.handle === handle)) {
        const msg = "The %TAG directive must only be given at most once per handle in the same document.";
        throw new PlainValue.YAMLSemanticError(directive, msg);
      }
      return {
        handle,
        prefix
      };
    }
    function resolveYamlDirective(doc, directive) {
      let [version] = directive.parameters;
      if (directive.name === "YAML:1.0") version = "1.0";
      if (!version) {
        const msg = "Insufficient parameters given for %YAML directive";
        throw new PlainValue.YAMLSemanticError(directive, msg);
      }
      if (!documentOptions[version]) {
        const v0 = doc.version || doc.options.version;
        const msg = `Document will be parsed as YAML ${v0} rather than YAML ${version}`;
        doc.warnings.push(new PlainValue.YAMLWarning(directive, msg));
      }
      return version;
    }
    function parseDirectives(doc, directives, prevDoc) {
      const directiveComments = [];
      let hasDirectives = false;
      for (const directive of directives) {
        const {
          comment,
          name
        } = directive;
        switch (name) {
          case "TAG":
            try {
              doc.tagPrefixes.push(resolveTagDirective(doc, directive));
            } catch (error) {
              doc.errors.push(error);
            }
            hasDirectives = true;
            break;
          case "YAML":
          case "YAML:1.0":
            if (doc.version) {
              const msg = "The %YAML directive must only be given at most once per document.";
              doc.errors.push(new PlainValue.YAMLSemanticError(directive, msg));
            }
            try {
              doc.version = resolveYamlDirective(doc, directive);
            } catch (error) {
              doc.errors.push(error);
            }
            hasDirectives = true;
            break;
          default:
            if (name) {
              const msg = `YAML only supports %TAG and %YAML directives, and not %${name}`;
              doc.warnings.push(new PlainValue.YAMLWarning(directive, msg));
            }
        }
        if (comment) directiveComments.push(comment);
      }
      if (prevDoc && !hasDirectives && "1.1" === (doc.version || prevDoc.version || doc.options.version)) {
        const copyTagPrefix = ({
          handle,
          prefix
        }) => ({
          handle,
          prefix
        });
        doc.tagPrefixes = prevDoc.tagPrefixes.map(copyTagPrefix);
        doc.version = prevDoc.version;
      }
      doc.commentBefore = directiveComments.join("\n") || null;
    }
    function assertCollection(contents) {
      if (contents instanceof resolveSeq.Collection) return true;
      throw new Error("Expected a YAML collection as document contents");
    }
    var Document = class _Document {
      constructor(options) {
        this.anchors = new Anchors(options.anchorPrefix);
        this.commentBefore = null;
        this.comment = null;
        this.contents = null;
        this.directivesEndMarker = null;
        this.errors = [];
        this.options = options;
        this.schema = null;
        this.tagPrefixes = [];
        this.version = null;
        this.warnings = [];
      }
      add(value) {
        assertCollection(this.contents);
        return this.contents.add(value);
      }
      addIn(path, value) {
        assertCollection(this.contents);
        this.contents.addIn(path, value);
      }
      delete(key) {
        assertCollection(this.contents);
        return this.contents.delete(key);
      }
      deleteIn(path) {
        if (resolveSeq.isEmptyPath(path)) {
          if (this.contents == null) return false;
          this.contents = null;
          return true;
        }
        assertCollection(this.contents);
        return this.contents.deleteIn(path);
      }
      getDefaults() {
        return _Document.defaults[this.version] || _Document.defaults[this.options.version] || {};
      }
      get(key, keepScalar) {
        return this.contents instanceof resolveSeq.Collection ? this.contents.get(key, keepScalar) : void 0;
      }
      getIn(path, keepScalar) {
        if (resolveSeq.isEmptyPath(path)) return !keepScalar && this.contents instanceof resolveSeq.Scalar ? this.contents.value : this.contents;
        return this.contents instanceof resolveSeq.Collection ? this.contents.getIn(path, keepScalar) : void 0;
      }
      has(key) {
        return this.contents instanceof resolveSeq.Collection ? this.contents.has(key) : false;
      }
      hasIn(path) {
        if (resolveSeq.isEmptyPath(path)) return this.contents !== void 0;
        return this.contents instanceof resolveSeq.Collection ? this.contents.hasIn(path) : false;
      }
      set(key, value) {
        assertCollection(this.contents);
        this.contents.set(key, value);
      }
      setIn(path, value) {
        if (resolveSeq.isEmptyPath(path)) this.contents = value;
        else {
          assertCollection(this.contents);
          this.contents.setIn(path, value);
        }
      }
      setSchema(id, customTags) {
        if (!id && !customTags && this.schema) return;
        if (typeof id === "number") id = id.toFixed(1);
        if (id === "1.0" || id === "1.1" || id === "1.2") {
          if (this.version) this.version = id;
          else this.options.version = id;
          delete this.options.schema;
        } else if (id && typeof id === "string") {
          this.options.schema = id;
        }
        if (Array.isArray(customTags)) this.options.customTags = customTags;
        const opt = Object.assign({}, this.getDefaults(), this.options);
        this.schema = new Schema.Schema(opt);
      }
      parse(node, prevDoc) {
        if (this.options.keepCstNodes) this.cstNode = node;
        if (this.options.keepNodeTypes) this.type = "DOCUMENT";
        const {
          directives = [],
          contents = [],
          directivesEndMarker,
          error,
          valueRange
        } = node;
        if (error) {
          if (!error.source) error.source = this;
          this.errors.push(error);
        }
        parseDirectives(this, directives, prevDoc);
        if (directivesEndMarker) this.directivesEndMarker = true;
        this.range = valueRange ? [valueRange.start, valueRange.end] : null;
        this.setSchema();
        this.anchors._cstAliases = [];
        parseContents(this, contents);
        this.anchors.resolveNodes();
        if (this.options.prettyErrors) {
          for (const error2 of this.errors) if (error2 instanceof PlainValue.YAMLError) error2.makePretty();
          for (const warn of this.warnings) if (warn instanceof PlainValue.YAMLError) warn.makePretty();
        }
        return this;
      }
      listNonDefaultTags() {
        return listTagNames(this.contents).filter((t) => t.indexOf(Schema.Schema.defaultPrefix) !== 0);
      }
      setTagPrefix(handle, prefix) {
        if (handle[0] !== "!" || handle[handle.length - 1] !== "!") throw new Error("Handle must start and end with !");
        if (prefix) {
          const prev = this.tagPrefixes.find((p) => p.handle === handle);
          if (prev) prev.prefix = prefix;
          else this.tagPrefixes.push({
            handle,
            prefix
          });
        } else {
          this.tagPrefixes = this.tagPrefixes.filter((p) => p.handle !== handle);
        }
      }
      toJSON(arg, onAnchor) {
        const {
          keepBlobsInJSON,
          mapAsMap,
          maxAliasCount
        } = this.options;
        const keep = keepBlobsInJSON && (typeof arg !== "string" || !(this.contents instanceof resolveSeq.Scalar));
        const ctx = {
          doc: this,
          indentStep: "  ",
          keep,
          mapAsMap: keep && !!mapAsMap,
          maxAliasCount,
          stringify
          // Requiring directly in Pair would create circular dependencies
        };
        const anchorNames = Object.keys(this.anchors.map);
        if (anchorNames.length > 0) ctx.anchors = new Map(anchorNames.map((name) => [this.anchors.map[name], {
          alias: [],
          aliasCount: 0,
          count: 1
        }]));
        const res = resolveSeq.toJSON(this.contents, arg, ctx);
        if (typeof onAnchor === "function" && ctx.anchors) for (const {
          count,
          res: res2
        } of ctx.anchors.values()) onAnchor(res2, count);
        return res;
      }
      toString() {
        if (this.errors.length > 0) throw new Error("Document with errors cannot be stringified");
        const indentSize = this.options.indent;
        if (!Number.isInteger(indentSize) || indentSize <= 0) {
          const s = JSON.stringify(indentSize);
          throw new Error(`"indent" option must be a positive integer, not ${s}`);
        }
        this.setSchema();
        const lines = [];
        let hasDirectives = false;
        if (this.version) {
          let vd = "%YAML 1.2";
          if (this.schema.name === "yaml-1.1") {
            if (this.version === "1.0") vd = "%YAML:1.0";
            else if (this.version === "1.1") vd = "%YAML 1.1";
          }
          lines.push(vd);
          hasDirectives = true;
        }
        const tagNames = this.listNonDefaultTags();
        this.tagPrefixes.forEach(({
          handle,
          prefix
        }) => {
          if (tagNames.some((t) => t.indexOf(prefix) === 0)) {
            lines.push(`%TAG ${handle} ${prefix}`);
            hasDirectives = true;
          }
        });
        if (hasDirectives || this.directivesEndMarker) lines.push("---");
        if (this.commentBefore) {
          if (hasDirectives || !this.directivesEndMarker) lines.unshift("");
          lines.unshift(this.commentBefore.replace(/^/gm, "#"));
        }
        const ctx = {
          anchors: /* @__PURE__ */ Object.create(null),
          doc: this,
          indent: "",
          indentStep: " ".repeat(indentSize),
          stringify
          // Requiring directly in nodes would create circular dependencies
        };
        let chompKeep = false;
        let contentComment = null;
        if (this.contents) {
          if (this.contents instanceof resolveSeq.Node) {
            if (this.contents.spaceBefore && (hasDirectives || this.directivesEndMarker)) lines.push("");
            if (this.contents.commentBefore) lines.push(this.contents.commentBefore.replace(/^/gm, "#"));
            ctx.forceBlockIndent = !!this.comment;
            contentComment = this.contents.comment;
          }
          const onChompKeep = contentComment ? null : () => chompKeep = true;
          const body = stringify(this.contents, ctx, () => contentComment = null, onChompKeep);
          lines.push(resolveSeq.addComment(body, "", contentComment));
        } else if (this.contents !== void 0) {
          lines.push(stringify(this.contents, ctx));
        }
        if (this.comment) {
          if ((!chompKeep || contentComment) && lines[lines.length - 1] !== "") lines.push("");
          lines.push(this.comment.replace(/^/gm, "#"));
        }
        return lines.join("\n") + "\n";
      }
    };
    PlainValue._defineProperty(Document, "defaults", documentOptions);
    exports2.Document = Document;
    exports2.defaultOptions = defaultOptions;
    exports2.scalarOptions = scalarOptions;
  }
});

// node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/index.js
var require_dist = __commonJS({
  "node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/dist/index.js"(exports2) {
    "use strict";
    var parseCst = require_parse_cst();
    var Document$1 = require_Document_9b4560a1();
    var Schema = require_Schema_88e323a7();
    var PlainValue = require_PlainValue_ec8e588e();
    var warnings = require_warnings_1000a372();
    require_resolveSeq_d03cb037();
    function createNode(value, wrapScalars = true, tag) {
      if (tag === void 0 && typeof wrapScalars === "string") {
        tag = wrapScalars;
        wrapScalars = true;
      }
      const options = Object.assign({}, Document$1.Document.defaults[Document$1.defaultOptions.version], Document$1.defaultOptions);
      const schema = new Schema.Schema(options);
      return schema.createNode(value, wrapScalars, tag);
    }
    var Document = class extends Document$1.Document {
      constructor(options) {
        super(Object.assign({}, Document$1.defaultOptions, options));
      }
    };
    function parseAllDocuments(src, options) {
      const stream = [];
      let prev;
      for (const cstDoc of parseCst.parse(src)) {
        const doc = new Document(options);
        doc.parse(cstDoc, prev);
        stream.push(doc);
        prev = doc;
      }
      return stream;
    }
    function parseDocument(src, options) {
      const cst = parseCst.parse(src);
      const doc = new Document(options).parse(cst[0]);
      if (cst.length > 1) {
        const errMsg = "Source contains multiple documents; please use YAML.parseAllDocuments()";
        doc.errors.unshift(new PlainValue.YAMLSemanticError(cst[1], errMsg));
      }
      return doc;
    }
    function parse(src, options) {
      const doc = parseDocument(src, options);
      doc.warnings.forEach((warning) => warnings.warn(warning));
      if (doc.errors.length > 0) throw doc.errors[0];
      return doc.toJSON();
    }
    function stringify(value, options) {
      const doc = new Document(options);
      doc.contents = value;
      return String(doc);
    }
    var YAML = {
      createNode,
      defaultOptions: Document$1.defaultOptions,
      Document,
      parse,
      parseAllDocuments,
      parseCST: parseCst.parse,
      parseDocument,
      scalarOptions: Document$1.scalarOptions,
      stringify
    };
    exports2.YAML = YAML;
  }
});

// node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/index.js
var require_yaml = __commonJS({
  "node_modules/.pnpm/yaml@1.10.2/node_modules/yaml/index.js"(exports2, module2) {
    module2.exports = require_dist().YAML;
  }
});

// node_modules/.pnpm/reftools@1.1.9/node_modules/reftools/lib/jptr.js
var require_jptr = __commonJS({
  "node_modules/.pnpm/reftools@1.1.9/node_modules/reftools/lib/jptr.js"(exports2, module2) {
    "use strict";
    function jpescape(s) {
      return s.replace(/\~/g, "~0").replace(/\//g, "~1");
    }
    function jpunescape(s) {
      return s.replace(/\~1/g, "/").replace(/~0/g, "~");
    }
    function jptr(obj, prop, newValue) {
      if (typeof obj === "undefined") return false;
      if (!prop || typeof prop !== "string" || prop === "#") return typeof newValue !== "undefined" ? newValue : obj;
      if (prop.indexOf("#") >= 0) {
        let parts = prop.split("#");
        let uri = parts[0];
        if (uri) return false;
        prop = parts[1];
        prop = decodeURIComponent(prop.slice(1).split("+").join(" "));
      }
      if (prop.startsWith("/")) prop = prop.slice(1);
      let components = prop.split("/");
      for (let i = 0; i < components.length; i++) {
        components[i] = jpunescape(components[i]);
        let setAndLast = typeof newValue !== "undefined" && i == components.length - 1;
        let index = parseInt(components[i], 10);
        if (!Array.isArray(obj) || isNaN(index) || index.toString() !== components[i]) {
          index = Array.isArray(obj) && components[i] === "-" ? -2 : -1;
        } else {
          components[i] = i > 0 ? components[i - 1] : "";
        }
        if (index != -1 || obj && obj.hasOwnProperty(components[i])) {
          if (index >= 0) {
            if (setAndLast) {
              obj[index] = newValue;
            }
            obj = obj[index];
          } else if (index === -2) {
            if (setAndLast) {
              if (Array.isArray(obj)) {
                obj.push(newValue);
              }
              return newValue;
            } else return void 0;
          } else {
            if (setAndLast) {
              obj[components[i]] = newValue;
            }
            obj = obj[components[i]];
          }
        } else {
          if (typeof newValue !== "undefined" && typeof obj === "object" && !Array.isArray(obj)) {
            obj[components[i]] = setAndLast ? newValue : components[i + 1] === "0" || components[i + 1] === "-" ? [] : {};
            obj = obj[components[i]];
          } else return false;
        }
      }
      return obj;
    }
    module2.exports = {
      jptr,
      jpescape,
      jpunescape
    };
  }
});

// node_modules/.pnpm/reftools@1.1.9/node_modules/reftools/lib/isref.js
var require_isref = __commonJS({
  "node_modules/.pnpm/reftools@1.1.9/node_modules/reftools/lib/isref.js"(exports2, module2) {
    "use strict";
    function isRef(obj, key) {
      return key === "$ref" && (!!obj && typeof obj[key] === "string");
    }
    module2.exports = {
      isRef
    };
  }
});

// node_modules/.pnpm/reftools@1.1.9/node_modules/reftools/lib/clone.js
var require_clone = __commonJS({
  "node_modules/.pnpm/reftools@1.1.9/node_modules/reftools/lib/clone.js"(exports2, module2) {
    "use strict";
    function nop(obj) {
      return obj;
    }
    function clone(obj) {
      return JSON.parse(JSON.stringify(obj));
    }
    function shallowClone(obj) {
      let result = {};
      for (let p in obj) {
        if (obj.hasOwnProperty(p)) {
          result[p] = obj[p];
        }
      }
      return result;
    }
    function deepClone(obj) {
      let result = Array.isArray(obj) ? [] : {};
      for (let p in obj) {
        if (obj.hasOwnProperty(p) || Array.isArray(obj)) {
          result[p] = typeof obj[p] === "object" ? deepClone(obj[p]) : obj[p];
        }
      }
      return result;
    }
    function fastClone(obj) {
      return Object.assign({}, obj);
    }
    function circularClone(obj, hash) {
      if (!hash) hash = /* @__PURE__ */ new WeakMap();
      if (Object(obj) !== obj || obj instanceof Function) return obj;
      if (hash.has(obj)) return hash.get(obj);
      try {
        var result = new obj.constructor();
      } catch (e) {
        result = Object.create(Object.getPrototypeOf(obj));
      }
      hash.set(obj, result);
      return Object.assign(result, ...Object.keys(obj).map(
        (key) => ({ [key]: circularClone(obj[key], hash) })
      ));
    }
    module2.exports = {
      nop,
      clone,
      shallowClone,
      deepClone,
      fastClone,
      circularClone
    };
  }
});

// node_modules/.pnpm/reftools@1.1.9/node_modules/reftools/lib/recurse.js
var require_recurse = __commonJS({
  "node_modules/.pnpm/reftools@1.1.9/node_modules/reftools/lib/recurse.js"(exports2, module2) {
    "use strict";
    var jpescape = require_jptr().jpescape;
    function defaultState() {
      return {
        path: "#",
        depth: 0,
        pkey: "",
        parent: {},
        payload: {},
        seen: /* @__PURE__ */ new WeakMap(),
        identity: false,
        identityDetection: false
      };
    }
    function recurse(object, state, callback) {
      if (!state) state = { depth: 0 };
      if (!state.depth) {
        state = Object.assign({}, defaultState(), state);
      }
      if (typeof object !== "object") return;
      let oPath = state.path;
      for (let key in object) {
        state.key = key;
        state.path = state.path + "/" + encodeURIComponent(jpescape(key));
        state.identityPath = state.seen.get(object[key]);
        state.identity = typeof state.identityPath !== "undefined";
        if (object.hasOwnProperty(key)) {
          callback(object, key, state);
        }
        if (typeof object[key] === "object" && !state.identity) {
          if (state.identityDetection && !Array.isArray(object[key]) && object[key] !== null) {
            state.seen.set(object[key], state.path);
          }
          let newState = {};
          newState.parent = object;
          newState.path = state.path;
          newState.depth = state.depth ? state.depth + 1 : 1;
          newState.pkey = key;
          newState.payload = state.payload;
          newState.seen = state.seen;
          newState.identity = false;
          newState.identityDetection = state.identityDetection;
          recurse(object[key], newState, callback);
        }
        state.path = oPath;
      }
    }
    module2.exports = {
      recurse
    };
  }
});

// node_modules/.pnpm/reftools@1.1.9/node_modules/reftools/lib/dereference.js
var require_dereference = __commonJS({
  "node_modules/.pnpm/reftools@1.1.9/node_modules/reftools/lib/dereference.js"(exports2, module2) {
    "use strict";
    var recurse = require_recurse().recurse;
    var clone = require_clone().shallowClone;
    var jptr = require_jptr().jptr;
    var isRef = require_isref().isRef;
    var getLogger = function(options) {
      if (options && options.verbose) {
        return {
          warn: function() {
            var args = Array.prototype.slice.call(arguments);
            console.warn.apply(console, args);
          }
        };
      } else {
        return {
          warn: function() {
          }
        };
      }
    };
    function dereference(o, definitions, options) {
      if (!options) options = {};
      if (!options.cache) options.cache = {};
      if (!options.state) options.state = {};
      options.state.identityDetection = true;
      options.depth = options.depth ? options.depth + 1 : 1;
      let obj = options.depth > 1 ? o : clone(o);
      let container = { data: obj };
      let defs = options.depth > 1 ? definitions : clone(definitions);
      if (!options.master) options.master = obj;
      let logger = getLogger(options);
      let changes = 1;
      while (changes > 0) {
        changes = 0;
        recurse(container, options.state, function(obj2, key, state) {
          if (isRef(obj2, key)) {
            let $ref = obj2[key];
            changes++;
            if (!options.cache[$ref]) {
              let entry = {};
              entry.path = state.path.split("/$ref")[0];
              entry.key = $ref;
              logger.warn("Dereffing %s at %s", $ref, entry.path);
              entry.source = defs;
              entry.data = jptr(entry.source, entry.key);
              if (entry.data === false) {
                entry.data = jptr(options.master, entry.key);
                entry.source = options.master;
              }
              if (entry.data === false) {
                logger.warn("Missing $ref target", entry.key);
              }
              options.cache[$ref] = entry;
              entry.data = state.parent[state.pkey] = dereference(jptr(entry.source, entry.key), entry.source, options);
              if (options.$ref && typeof state.parent[state.pkey] === "object" && state.parent[state.pkey] !== null) state.parent[state.pkey][options.$ref] = $ref;
              entry.resolved = true;
            } else {
              let entry = options.cache[$ref];
              if (entry.resolved) {
                logger.warn("Patching %s for %s", $ref, entry.path);
                state.parent[state.pkey] = entry.data;
                if (options.$ref && typeof state.parent[state.pkey] === "object" && state.parent[state.pkey] !== null) state.parent[state.pkey][options.$ref] = $ref;
              } else if ($ref === entry.path) {
                throw new Error(`Tight circle at ${entry.path}`);
              } else {
                logger.warn("Unresolved ref");
                state.parent[state.pkey] = jptr(entry.source, entry.path);
                if (state.parent[state.pkey] === false) {
                  state.parent[state.pkey] = jptr(entry.source, entry.key);
                }
                if (options.$ref && typeof state.parent[state.pkey] === "object" && state.parent[state.pkey] !== null) state.parent[options.$ref] = $ref;
              }
            }
          }
        });
      }
      return container.data;
    }
    module2.exports = {
      dereference
    };
  }
});

// node_modules/.pnpm/fast-safe-stringify@2.1.1/node_modules/fast-safe-stringify/index.js
var require_fast_safe_stringify = __commonJS({
  "node_modules/.pnpm/fast-safe-stringify@2.1.1/node_modules/fast-safe-stringify/index.js"(exports2, module2) {
    module2.exports = stringify;
    stringify.default = stringify;
    stringify.stable = deterministicStringify;
    stringify.stableStringify = deterministicStringify;
    var LIMIT_REPLACE_NODE = "[...]";
    var CIRCULAR_REPLACE_NODE = "[Circular]";
    var arr = [];
    var replacerStack = [];
    function defaultOptions() {
      return {
        depthLimit: Number.MAX_SAFE_INTEGER,
        edgesLimit: Number.MAX_SAFE_INTEGER
      };
    }
    function stringify(obj, replacer, spacer, options) {
      if (typeof options === "undefined") {
        options = defaultOptions();
      }
      decirc(obj, "", 0, [], void 0, 0, options);
      var res;
      try {
        if (replacerStack.length === 0) {
          res = JSON.stringify(obj, replacer, spacer);
        } else {
          res = JSON.stringify(obj, replaceGetterValues(replacer), spacer);
        }
      } catch (_) {
        return JSON.stringify("[unable to serialize, circular reference is too complex to analyze]");
      } finally {
        while (arr.length !== 0) {
          var part = arr.pop();
          if (part.length === 4) {
            Object.defineProperty(part[0], part[1], part[3]);
          } else {
            part[0][part[1]] = part[2];
          }
        }
      }
      return res;
    }
    function setReplace(replace, val, k, parent) {
      var propertyDescriptor = Object.getOwnPropertyDescriptor(parent, k);
      if (propertyDescriptor.get !== void 0) {
        if (propertyDescriptor.configurable) {
          Object.defineProperty(parent, k, { value: replace });
          arr.push([parent, k, val, propertyDescriptor]);
        } else {
          replacerStack.push([val, k, replace]);
        }
      } else {
        parent[k] = replace;
        arr.push([parent, k, val]);
      }
    }
    function decirc(val, k, edgeIndex, stack, parent, depth, options) {
      depth += 1;
      var i;
      if (typeof val === "object" && val !== null) {
        for (i = 0; i < stack.length; i++) {
          if (stack[i] === val) {
            setReplace(CIRCULAR_REPLACE_NODE, val, k, parent);
            return;
          }
        }
        if (typeof options.depthLimit !== "undefined" && depth > options.depthLimit) {
          setReplace(LIMIT_REPLACE_NODE, val, k, parent);
          return;
        }
        if (typeof options.edgesLimit !== "undefined" && edgeIndex + 1 > options.edgesLimit) {
          setReplace(LIMIT_REPLACE_NODE, val, k, parent);
          return;
        }
        stack.push(val);
        if (Array.isArray(val)) {
          for (i = 0; i < val.length; i++) {
            decirc(val[i], i, i, stack, val, depth, options);
          }
        } else {
          var keys = Object.keys(val);
          for (i = 0; i < keys.length; i++) {
            var key = keys[i];
            decirc(val[key], key, i, stack, val, depth, options);
          }
        }
        stack.pop();
      }
    }
    function compareFunction(a, b) {
      if (a < b) {
        return -1;
      }
      if (a > b) {
        return 1;
      }
      return 0;
    }
    function deterministicStringify(obj, replacer, spacer, options) {
      if (typeof options === "undefined") {
        options = defaultOptions();
      }
      var tmp = deterministicDecirc(obj, "", 0, [], void 0, 0, options) || obj;
      var res;
      try {
        if (replacerStack.length === 0) {
          res = JSON.stringify(tmp, replacer, spacer);
        } else {
          res = JSON.stringify(tmp, replaceGetterValues(replacer), spacer);
        }
      } catch (_) {
        return JSON.stringify("[unable to serialize, circular reference is too complex to analyze]");
      } finally {
        while (arr.length !== 0) {
          var part = arr.pop();
          if (part.length === 4) {
            Object.defineProperty(part[0], part[1], part[3]);
          } else {
            part[0][part[1]] = part[2];
          }
        }
      }
      return res;
    }
    function deterministicDecirc(val, k, edgeIndex, stack, parent, depth, options) {
      depth += 1;
      var i;
      if (typeof val === "object" && val !== null) {
        for (i = 0; i < stack.length; i++) {
          if (stack[i] === val) {
            setReplace(CIRCULAR_REPLACE_NODE, val, k, parent);
            return;
          }
        }
        try {
          if (typeof val.toJSON === "function") {
            return;
          }
        } catch (_) {
          return;
        }
        if (typeof options.depthLimit !== "undefined" && depth > options.depthLimit) {
          setReplace(LIMIT_REPLACE_NODE, val, k, parent);
          return;
        }
        if (typeof options.edgesLimit !== "undefined" && edgeIndex + 1 > options.edgesLimit) {
          setReplace(LIMIT_REPLACE_NODE, val, k, parent);
          return;
        }
        stack.push(val);
        if (Array.isArray(val)) {
          for (i = 0; i < val.length; i++) {
            deterministicDecirc(val[i], i, i, stack, val, depth, options);
          }
        } else {
          var tmp = {};
          var keys = Object.keys(val).sort(compareFunction);
          for (i = 0; i < keys.length; i++) {
            var key = keys[i];
            deterministicDecirc(val[key], key, i, stack, val, depth, options);
            tmp[key] = val[key];
          }
          if (typeof parent !== "undefined") {
            arr.push([parent, k, val]);
            parent[k] = tmp;
          } else {
            return tmp;
          }
        }
        stack.pop();
      }
    }
    function replaceGetterValues(replacer) {
      replacer = typeof replacer !== "undefined" ? replacer : function(k, v) {
        return v;
      };
      return function(key, val) {
        if (replacerStack.length > 0) {
          for (var i = 0; i < replacerStack.length; i++) {
            var part = replacerStack[i];
            if (part[1] === key && part[0] === val) {
              val = part[2];
              replacerStack.splice(i, 1);
              break;
            }
          }
        }
        return replacer.call(this, key, val);
      };
    }
  }
});

// node_modules/.pnpm/oas-kit-common@1.0.8/node_modules/oas-kit-common/index.js
var require_oas_kit_common = __commonJS({
  "node_modules/.pnpm/oas-kit-common@1.0.8/node_modules/oas-kit-common/index.js"(exports2, module2) {
    "use strict";
    var sjs = require_fast_safe_stringify();
    var colour = process.env.NODE_DISABLE_COLORS ? { red: "", yellow: "", green: "", normal: "" } : { red: "\x1B[31m", yellow: "\x1B[33;1m", green: "\x1B[32m", normal: "\x1B[0m" };
    function uniqueOnly(value, index, self) {
      return self.indexOf(value) === index;
    }
    function hasDuplicates(array) {
      return new Set(array).size !== array.length;
    }
    function allSame(array) {
      return new Set(array).size <= 1;
    }
    function deepEquals(obj1, obj2) {
      function _equals(obj12, obj22) {
        return sjs.stringify(obj12) === sjs.stringify(Object.assign({}, obj12, obj22));
      }
      return _equals(obj1, obj2) && _equals(obj2, obj1);
    }
    function compressArray(arr) {
      let result = [];
      for (let candidate of arr) {
        let dupe = result.find(function(e, i, a) {
          return deepEquals(e, candidate);
        });
        if (!dupe) result.push(candidate);
      }
      return result;
    }
    function distinctArray(arr) {
      return arr.length === compressArray(arr).length;
    }
    function firstDupe(arr) {
      return arr.find(function(e, i, a) {
        return arr.indexOf(e) < i;
      });
    }
    function hash(s) {
      let hash2 = 0;
      let chr;
      if (s.length === 0) return hash2;
      for (let i = 0; i < s.length; i++) {
        chr = s.charCodeAt(i);
        hash2 = (hash2 << 5) - hash2 + chr;
        hash2 |= 0;
      }
      return hash2;
    }
    String.prototype.toCamelCase = function camelize() {
      return this.toLowerCase().replace(/[-_ \/\.](.)/g, function(match, group1) {
        return group1.toUpperCase();
      });
    };
    var parameterTypeProperties = [
      "format",
      "minimum",
      "maximum",
      "exclusiveMinimum",
      "exclusiveMaximum",
      "minLength",
      "maxLength",
      "multipleOf",
      "minItems",
      "maxItems",
      "uniqueItems",
      "minProperties",
      "maxProperties",
      "additionalProperties",
      "pattern",
      "enum",
      "default"
    ];
    var arrayProperties = [
      "items",
      "minItems",
      "maxItems",
      "uniqueItems"
    ];
    var httpMethods = [
      "get",
      "post",
      "put",
      "delete",
      "patch",
      "head",
      "options",
      "trace"
    ];
    function sanitise(s) {
      s = s.replace("[]", "Array");
      let components = s.split("/");
      components[0] = components[0].replace(/[^A-Za-z0-9_\-\.]+|\s+/gm, "_");
      return components.join("/");
    }
    function sanitiseAll(s) {
      return sanitise(s.split("/").join("_"));
    }
    module2.exports = {
      colour,
      uniqueOnly,
      hasDuplicates,
      allSame,
      distinctArray,
      firstDupe,
      hash,
      parameterTypeProperties,
      arrayProperties,
      httpMethods,
      sanitise,
      sanitiseAll
    };
  }
});

// node_modules/.pnpm/oas-resolver@2.5.6/node_modules/oas-resolver/index.js
var require_oas_resolver = __commonJS({
  "node_modules/.pnpm/oas-resolver@2.5.6/node_modules/oas-resolver/index.js"(exports2, module2) {
    "use strict";
    var fs = require("fs");
    var path = require("path");
    var url = require("url");
    var fetch2 = require_lib2();
    var yaml = require_yaml();
    var jptr = require_jptr().jptr;
    var recurse = require_recurse().recurse;
    var clone = require_clone().clone;
    var deRef = require_dereference().dereference;
    var isRef = require_isref().isRef;
    var common = require_oas_kit_common();
    function unique(arr) {
      return [...new Set(arr)];
    }
    function readFileAsync(filename, encoding, options, pointer, def) {
      return new Promise(function(resolve2, reject) {
        fs.readFile(filename, encoding, function(err, data) {
          if (err) {
            if (options.ignoreIOErrors && def) {
              if (options.verbose) console.warn("FAILED", pointer);
              options.externalRefs[pointer].failed = true;
              resolve2(def);
            } else {
              reject(err);
            }
          } else {
            resolve2(data);
          }
        });
      });
    }
    function resolveAllFragment(obj, context, src, parentPath, base, options) {
      let attachPoint = options.externalRefs[src + parentPath].paths[0];
      let baseUrl = url.parse(base);
      let seen = {};
      let changes = 1;
      while (changes) {
        changes = 0;
        recurse(obj, { identityDetection: true }, function(obj2, key, state) {
          if (isRef(obj2, key)) {
            if (obj2[key].startsWith("#")) {
              if (!seen[obj2[key]] && !obj2.$fixed) {
                let target = clone(jptr(context, obj2[key]));
                if (options.verbose > 1) console.warn((target === false ? common.colour.red : common.colour.green) + "Fragment resolution", obj2[key], common.colour.normal);
                if (target === false) {
                  state.parent[state.pkey] = {};
                  if (options.fatal) {
                    let ex = new Error("Fragment $ref resolution failed " + obj2[key]);
                    if (options.promise) options.promise.reject(ex);
                    else throw ex;
                  }
                } else {
                  changes++;
                  state.parent[state.pkey] = target;
                  seen[obj2[key]] = state.path.replace("/%24ref", "");
                }
              } else {
                if (!obj2.$fixed) {
                  let newRef = (attachPoint + "/" + seen[obj2[key]]).split("/#/").join("/");
                  state.parent[state.pkey] = { $ref: newRef, "x-miro": obj2[key], $fixed: true };
                  if (options.verbose > 1) console.warn("Replacing with", newRef);
                  changes++;
                }
              }
            } else if (baseUrl.protocol) {
              let newRef = url.resolve(base, obj2[key]).toString();
              if (options.verbose > 1) console.warn(common.colour.yellow + "Rewriting external url ref", obj2[key], "as", newRef, common.colour.normal);
              obj2["x-miro"] = obj2[key];
              if (options.externalRefs[obj2[key]]) {
                if (!options.externalRefs[newRef]) {
                  options.externalRefs[newRef] = options.externalRefs[obj2[key]];
                }
                options.externalRefs[newRef].failed = options.externalRefs[obj2[key]].failed;
              }
              obj2[key] = newRef;
            } else if (!obj2["x-miro"]) {
              let newRef = url.resolve(base, obj2[key]).toString();
              let failed = false;
              if (options.externalRefs[obj2[key]]) {
                failed = options.externalRefs[obj2[key]].failed;
              }
              if (!failed) {
                if (options.verbose > 1) console.warn(common.colour.yellow + "Rewriting external ref", obj2[key], "as", newRef, common.colour.normal);
                obj2["x-miro"] = obj2[key];
                obj2[key] = newRef;
              }
            }
          }
        });
      }
      recurse(obj, {}, function(obj2, key, state) {
        if (isRef(obj2, key)) {
          if (typeof obj2.$fixed !== "undefined") delete obj2.$fixed;
        }
      });
      if (options.verbose > 1) console.warn("Finished fragment resolution");
      return obj;
    }
    function filterData(data, options) {
      if (!options.filters || !options.filters.length) return data;
      for (let filter of options.filters) {
        data = filter(data, options);
      }
      return data;
    }
    function testProtocol(input, backup) {
      if (input && input.length > 2) return input;
      if (backup && backup.length > 2) return backup;
      return "file:";
    }
    function resolveExternal(root, pointer, options, callback) {
      var u = url.parse(options.source);
      var base = options.source.split("\\").join("/").split("/");
      let doc = base.pop();
      if (!doc) base.pop();
      let fragment = "";
      let fnComponents = pointer.split("#");
      if (fnComponents.length > 1) {
        fragment = "#" + fnComponents[1];
        pointer = fnComponents[0];
      }
      base = base.join("/");
      let u2 = url.parse(pointer);
      let effectiveProtocol = testProtocol(u2.protocol, u.protocol);
      let target;
      if (effectiveProtocol === "file:") {
        target = path.resolve(base ? base + "/" : "", pointer);
      } else {
        target = url.resolve(base ? base + "/" : "", pointer);
      }
      if (options.cache[target]) {
        if (options.verbose) console.warn("CACHED", target, fragment);
        let context = clone(options.cache[target]);
        let data = options.externalRef = context;
        if (fragment) {
          data = jptr(data, fragment);
          if (data === false) {
            data = {};
            if (options.fatal) {
              let ex = new Error("Cached $ref resolution failed " + target + fragment);
              if (options.promise) options.promise.reject(ex);
              else throw ex;
            }
          }
        }
        data = resolveAllFragment(data, context, pointer, fragment, target, options);
        data = filterData(data, options);
        callback(clone(data), target, options);
        return Promise.resolve(data);
      }
      if (options.verbose) console.warn("GET", target, fragment);
      if (options.handlers && options.handlers[effectiveProtocol]) {
        return options.handlers[effectiveProtocol](base, pointer, fragment, options).then(function(data) {
          options.externalRef = data;
          data = filterData(data, options);
          options.cache[target] = data;
          callback(data, target, options);
          return data;
        }).catch(function(ex) {
          if (options.verbose) console.warn(ex);
          throw ex;
        });
      } else if (effectiveProtocol && effectiveProtocol.startsWith("http")) {
        const fetchOptions = Object.assign({}, options.fetchOptions, { agent: options.agent });
        return options.fetch(target, fetchOptions).then(function(res) {
          if (res.status !== 200) {
            if (options.ignoreIOErrors) {
              if (options.verbose) console.warn("FAILED", pointer);
              options.externalRefs[pointer].failed = true;
              return '{"$ref":"' + pointer + '"}';
            } else {
              throw new Error(`Received status code ${res.status}: ${target}`);
            }
          }
          return res.text();
        }).then(function(data) {
          try {
            let context = yaml.parse(data, { schema: "core", prettyErrors: true });
            data = options.externalRef = context;
            options.cache[target] = clone(data);
            if (fragment) {
              data = jptr(data, fragment);
              if (data === false) {
                data = {};
                if (options.fatal) {
                  let ex = new Error("Remote $ref resolution failed " + target + fragment);
                  if (options.promise) options.promise.reject(ex);
                  else throw ex;
                }
              }
            }
            data = resolveAllFragment(data, context, pointer, fragment, target, options);
            data = filterData(data, options);
          } catch (ex) {
            if (options.verbose) console.warn(ex);
            if (options.promise && options.fatal) options.promise.reject(ex);
            else throw ex;
          }
          callback(data, target, options);
          return data;
        }).catch(function(err) {
          if (options.verbose) console.warn(err);
          options.cache[target] = {};
          if (options.promise && options.fatal) options.promise.reject(err);
          else throw err;
        });
      } else {
        const def = '{"$ref":"' + pointer + '"}';
        return readFileAsync(target, options.encoding || "utf8", options, pointer, def).then(function(data) {
          try {
            let context = yaml.parse(data, { schema: "core", prettyErrors: true });
            data = options.externalRef = context;
            options.cache[target] = clone(data);
            if (fragment) {
              data = jptr(data, fragment);
              if (data === false) {
                data = {};
                if (options.fatal) {
                  let ex = new Error("File $ref resolution failed " + target + fragment);
                  if (options.promise) options.promise.reject(ex);
                  else throw ex;
                }
              }
            }
            data = resolveAllFragment(data, context, pointer, fragment, target, options);
            data = filterData(data, options);
          } catch (ex) {
            if (options.verbose) console.warn(ex);
            if (options.promise && options.fatal) options.promise.reject(ex);
            else throw ex;
          }
          callback(data, target, options);
          return data;
        }).catch(function(err) {
          if (options.verbose) console.warn(err);
          if (options.promise && options.fatal) options.promise.reject(err);
          else throw err;
        });
      }
    }
    function scanExternalRefs(options) {
      return new Promise(function(res, rej) {
        function inner(obj, key, state) {
          if (obj[key] && isRef(obj[key], "$ref")) {
            let $ref = obj[key].$ref;
            if (!$ref.startsWith("#")) {
              let $extra = "";
              if (!refs[$ref]) {
                let potential = Object.keys(refs).find(function(e, i, a) {
                  return $ref.startsWith(e + "/");
                });
                if (potential) {
                  if (options.verbose) console.warn("Found potential subschema at", potential);
                  $extra = "/" + ($ref.split("#")[1] || "").replace(potential.split("#")[1] || "");
                  $extra = $extra.split("/undefined").join("");
                  $ref = potential;
                }
              }
              if (!refs[$ref]) {
                refs[$ref] = { resolved: false, paths: [], extras: {}, description: obj[key].description };
              }
              if (refs[$ref].resolved) {
                if (refs[$ref].failed) {
                } else if (options.rewriteRefs) {
                  let newRef = refs[$ref].resolvedAt;
                  if (options.verbose > 1) console.warn("Rewriting ref", $ref, newRef);
                  obj[key]["x-miro"] = $ref;
                  obj[key].$ref = newRef + $extra;
                } else {
                  obj[key] = clone(refs[$ref].data);
                }
              } else {
                refs[$ref].paths.push(state.path);
                refs[$ref].extras[state.path] = $extra;
              }
            }
          }
        }
        let refs = options.externalRefs;
        if (options.resolver.depth > 0 && options.source === options.resolver.base) {
          return res(refs);
        }
        recurse(options.openapi.definitions, { identityDetection: true, path: "#/definitions" }, inner);
        recurse(options.openapi.components, { identityDetection: true, path: "#/components" }, inner);
        recurse(options.openapi, { identityDetection: true }, inner);
        res(refs);
      });
    }
    function findExternalRefs(options) {
      return new Promise(function(res, rej) {
        scanExternalRefs(options).then(function(refs) {
          for (let ref in refs) {
            if (!refs[ref].resolved) {
              let depth = options.resolver.depth;
              if (depth > 0) depth++;
              options.resolver.actions[depth].push(function() {
                return resolveExternal(options.openapi, ref, options, function(data, source, options2) {
                  if (!refs[ref].resolved) {
                    let external = {};
                    external.context = refs[ref];
                    external.$ref = ref;
                    external.original = clone(data);
                    external.updated = data;
                    external.source = source;
                    options2.externals.push(external);
                    refs[ref].resolved = true;
                  }
                  let localOptions = Object.assign({}, options2, {
                    source: "",
                    resolver: {
                      actions: options2.resolver.actions,
                      depth: options2.resolver.actions.length - 1,
                      base: options2.resolver.base
                    }
                  });
                  if (options2.patch && refs[ref].description && !data.description && typeof data === "object") {
                    data.description = refs[ref].description;
                  }
                  refs[ref].data = data;
                  let pointers = unique(refs[ref].paths);
                  pointers = pointers.sort(function(a, b) {
                    const aComp = a.startsWith("#/components/") || a.startsWith("#/definitions/");
                    const bComp = b.startsWith("#/components/") || b.startsWith("#/definitions/");
                    if (aComp && !bComp) return -1;
                    if (bComp && !aComp) return 1;
                    return 0;
                  });
                  for (let ptr of pointers) {
                    if (refs[ref].resolvedAt && ptr !== refs[ref].resolvedAt && ptr.indexOf("x-ms-examples/") < 0) {
                      if (options2.verbose > 1) console.warn("Creating pointer to data at", ptr);
                      jptr(options2.openapi, ptr, { $ref: refs[ref].resolvedAt + refs[ref].extras[ptr], "x-miro": ref + refs[ref].extras[ptr] });
                    } else {
                      if (refs[ref].resolvedAt) {
                        if (options2.verbose > 1) console.warn("Avoiding circular reference");
                      } else {
                        refs[ref].resolvedAt = ptr;
                        if (options2.verbose > 1) console.warn("Creating initial clone of data at", ptr);
                      }
                      let cdata = clone(data);
                      jptr(options2.openapi, ptr, cdata);
                    }
                  }
                  if (options2.resolver.actions[localOptions.resolver.depth].length === 0) {
                    options2.resolver.actions[localOptions.resolver.depth].push(function() {
                      return findExternalRefs(localOptions);
                    });
                  }
                });
              });
            }
          }
        }).catch(function(ex) {
          if (options.verbose) console.warn(ex);
          rej(ex);
        });
        let result = { options };
        result.actions = options.resolver.actions[options.resolver.depth];
        res(result);
      });
    }
    var serial = (funcs) => funcs.reduce((promise, func) => promise.then((result) => func().then(Array.prototype.concat.bind(result))), Promise.resolve([]));
    function loopReferences(options, res, rej) {
      options.resolver.actions.push([]);
      findExternalRefs(options).then(function(data) {
        serial(data.actions).then(function() {
          if (options.resolver.depth >= options.resolver.actions.length) {
            console.warn("Ran off the end of resolver actions");
            return res(true);
          } else {
            options.resolver.depth++;
            if (options.resolver.actions[options.resolver.depth].length) {
              setTimeout(function() {
                loopReferences(data.options, res, rej);
              }, 0);
            } else {
              if (options.verbose > 1) console.warn(common.colour.yellow + "Finished external resolution!", common.colour.normal);
              if (options.resolveInternal) {
                if (options.verbose > 1) console.warn(common.colour.yellow + "Starting internal resolution!", common.colour.normal);
                options.openapi = deRef(options.openapi, options.original, { verbose: options.verbose - 1 });
                if (options.verbose > 1) console.warn(common.colour.yellow + "Finished internal resolution!", common.colour.normal);
              }
              recurse(options.openapi, {}, function(obj, key, state) {
                if (isRef(obj, key)) {
                  if (!options.preserveMiro) delete obj["x-miro"];
                }
              });
              res(options);
            }
          }
        }).catch(function(ex) {
          if (options.verbose) console.warn(ex);
          rej(ex);
        });
      }).catch(function(ex) {
        if (options.verbose) console.warn(ex);
        rej(ex);
      });
    }
    function setupOptions(options) {
      if (!options.cache) options.cache = {};
      if (!options.fetch) options.fetch = fetch2;
      if (options.source) {
        let srcUrl = url.parse(options.source);
        if (!srcUrl.protocol || srcUrl.protocol.length <= 2) {
          options.source = path.resolve(options.source);
        }
      }
      options.externals = [];
      options.externalRefs = {};
      options.rewriteRefs = true;
      options.resolver = {};
      options.resolver.depth = 0;
      options.resolver.base = options.source;
      options.resolver.actions = [[]];
    }
    function optionalResolve(options) {
      setupOptions(options);
      return new Promise(function(res, rej) {
        if (options.resolve)
          loopReferences(options, res, rej);
        else
          res(options);
      });
    }
    function resolve(openapi, source, options) {
      if (!options) options = {};
      options.openapi = openapi;
      options.source = source;
      options.resolve = true;
      setupOptions(options);
      return new Promise(function(res, rej) {
        loopReferences(options, res, rej);
      });
    }
    module2.exports = {
      optionalResolve,
      resolve
    };
  }
});

// node_modules/.pnpm/oas-schema-walker@1.1.5/node_modules/oas-schema-walker/index.js
var require_oas_schema_walker = __commonJS({
  "node_modules/.pnpm/oas-schema-walker@1.1.5/node_modules/oas-schema-walker/index.js"(exports2, module2) {
    "use strict";
    function getDefaultState() {
      return { depth: 0, seen: /* @__PURE__ */ new WeakMap(), top: true, combine: false, allowRefSiblings: false };
    }
    function walkSchema(schema, parent, state, callback) {
      if (typeof state.depth === "undefined") state = getDefaultState();
      if (schema === null || typeof schema === "undefined") return schema;
      if (typeof schema.$ref !== "undefined") {
        let temp = { $ref: schema.$ref };
        if (state.allowRefSiblings && schema.description) {
          temp.description = schema.description;
        }
        callback(temp, parent, state);
        return temp;
      }
      if (state.combine) {
        if (schema.allOf && Array.isArray(schema.allOf) && schema.allOf.length === 1) {
          schema = Object.assign({}, schema.allOf[0], schema);
          delete schema.allOf;
        }
        if (schema.anyOf && Array.isArray(schema.anyOf) && schema.anyOf.length === 1) {
          schema = Object.assign({}, schema.anyOf[0], schema);
          delete schema.anyOf;
        }
        if (schema.oneOf && Array.isArray(schema.oneOf) && schema.oneOf.length === 1) {
          schema = Object.assign({}, schema.oneOf[0], schema);
          delete schema.oneOf;
        }
      }
      callback(schema, parent, state);
      if (state.seen.has(schema)) {
        return schema;
      }
      if (typeof schema === "object" && schema !== null) state.seen.set(schema, true);
      state.top = false;
      state.depth++;
      if (typeof schema.items !== "undefined") {
        state.property = "items";
        walkSchema(schema.items, schema, state, callback);
      }
      if (schema.additionalItems) {
        if (typeof schema.additionalItems === "object") {
          state.property = "additionalItems";
          walkSchema(schema.additionalItems, schema, state, callback);
        }
      }
      if (schema.additionalProperties) {
        if (typeof schema.additionalProperties === "object") {
          state.property = "additionalProperties";
          walkSchema(schema.additionalProperties, schema, state, callback);
        }
      }
      if (schema.properties) {
        for (let prop in schema.properties) {
          let subSchema = schema.properties[prop];
          state.property = "properties/" + prop;
          walkSchema(subSchema, schema, state, callback);
        }
      }
      if (schema.patternProperties) {
        for (let prop in schema.patternProperties) {
          let subSchema = schema.patternProperties[prop];
          state.property = "patternProperties/" + prop;
          walkSchema(subSchema, schema, state, callback);
        }
      }
      if (schema.allOf) {
        for (let index in schema.allOf) {
          let subSchema = schema.allOf[index];
          state.property = "allOf/" + index;
          walkSchema(subSchema, schema, state, callback);
        }
      }
      if (schema.anyOf) {
        for (let index in schema.anyOf) {
          let subSchema = schema.anyOf[index];
          state.property = "anyOf/" + index;
          walkSchema(subSchema, schema, state, callback);
        }
      }
      if (schema.oneOf) {
        for (let index in schema.oneOf) {
          let subSchema = schema.oneOf[index];
          state.property = "oneOf/" + index;
          walkSchema(subSchema, schema, state, callback);
        }
      }
      if (schema.not) {
        state.property = "not";
        walkSchema(schema.not, schema, state, callback);
      }
      state.depth--;
      return schema;
    }
    module2.exports = {
      getDefaultState,
      walkSchema
    };
  }
});

// node_modules/.pnpm/swagger2openapi@7.0.8/node_modules/swagger2openapi/lib/statusCodes.js
var require_statusCodes = __commonJS({
  "node_modules/.pnpm/swagger2openapi@7.0.8/node_modules/swagger2openapi/lib/statusCodes.js"(exports2, module2) {
    "use strict";
    var http = require("http");
    var ours = {
      "default": "Default response",
      "1XX": "Informational",
      "103": "Early hints",
      // not in Node < 10
      "2XX": "Successful",
      "3XX": "Redirection",
      "4XX": "Client Error",
      "5XX": "Server Error",
      "7XX": "Developer Error"
      // April fools RFC
    };
    module2.exports = {
      statusCodes: Object.assign({}, ours, http.STATUS_CODES)
    };
  }
});

// node_modules/.pnpm/swagger2openapi@7.0.8/node_modules/swagger2openapi/package.json
var require_package = __commonJS({
  "node_modules/.pnpm/swagger2openapi@7.0.8/node_modules/swagger2openapi/package.json"(exports2, module2) {
    module2.exports = {
      name: "swagger2openapi",
      version: "7.0.8",
      description: "Convert Swagger 2.0 definitions to OpenApi 3.0 and validate",
      main: "index.js",
      bin: {
        swagger2openapi: "./swagger2openapi.js",
        "oas-validate": "./oas-validate.js",
        boast: "./boast.js"
      },
      funding: "https://github.com/Mermade/oas-kit?sponsor=1",
      scripts: {
        test: "mocha"
      },
      browserify: {
        transform: [
          [
            "babelify",
            {
              presets: [
                "es2015"
              ]
            }
          ]
        ]
      },
      repository: {
        url: "https://github.com/Mermade/oas-kit.git",
        type: "git"
      },
      bugs: {
        url: "https://github.com/mermade/oas-kit/issues"
      },
      author: "Mike Ralphson <mike.ralphson@gmail.com>",
      license: "BSD-3-Clause",
      dependencies: {
        "call-me-maybe": "^1.0.1",
        "node-fetch": "^2.6.1",
        "node-fetch-h2": "^2.3.0",
        "node-readfiles": "^0.2.0",
        "oas-kit-common": "^1.0.8",
        "oas-resolver": "^2.5.6",
        "oas-schema-walker": "^1.1.5",
        "oas-validator": "^5.0.8",
        reftools: "^1.1.9",
        yaml: "^1.10.0",
        yargs: "^17.0.1"
      },
      keywords: [
        "swagger",
        "openapi",
        "openapi2",
        "openapi3",
        "converter",
        "conversion",
        "validator",
        "validation",
        "resolver",
        "lint",
        "linter"
      ],
      gitHead: "b1bba3fc5007e96a991bf2a015cf0534ac36b88b"
    };
  }
});

// node_modules/.pnpm/swagger2openapi@7.0.8/node_modules/swagger2openapi/index.js
var require_swagger2openapi = __commonJS({
  "node_modules/.pnpm/swagger2openapi@7.0.8/node_modules/swagger2openapi/index.js"(exports2, module2) {
    "use strict";
    var fs = require("fs");
    var url = require("url");
    var pathlib = require("path");
    var maybe = require_maybe();
    var fetch2 = require_lib2();
    var yaml = require_yaml();
    var jptr = require_jptr();
    var resolveInternal = jptr.jptr;
    var isRef = require_isref().isRef;
    var clone = require_clone().clone;
    var cclone = require_clone().circularClone;
    var recurse = require_recurse().recurse;
    var resolver = require_oas_resolver();
    var sw = require_oas_schema_walker();
    var common = require_oas_kit_common();
    var statusCodes = require_statusCodes().statusCodes;
    var ourVersion = require_package().version;
    var targetVersion = "3.0.0";
    var componentNames;
    var S2OError = class extends Error {
      constructor(message) {
        super(message);
        this.name = "S2OError";
      }
    };
    function throwError(message, options) {
      let err = new S2OError(message);
      err.options = options;
      if (options.promise) {
        options.promise.reject(err);
      } else {
        throw err;
      }
    }
    function throwOrWarn(message, container, options) {
      if (options.warnOnly) {
        container[options.warnProperty || "x-s2o-warning"] = message;
      } else {
        throwError(message, options);
      }
    }
    function fixUpSubSchema(schema, parent, options) {
      if (schema.nullable) options.patches++;
      if (schema.discriminator && typeof schema.discriminator === "string") {
        schema.discriminator = { propertyName: schema.discriminator };
      }
      if (schema.items && Array.isArray(schema.items)) {
        if (schema.items.length === 0) {
          schema.items = {};
        } else if (schema.items.length === 1) {
          schema.items = schema.items[0];
        } else schema.items = { anyOf: schema.items };
      }
      if (schema.type && Array.isArray(schema.type)) {
        if (options.patch) {
          options.patches++;
          if (schema.type.length === 0) {
            delete schema.type;
          } else {
            if (!schema.oneOf) schema.oneOf = [];
            for (let type of schema.type) {
              let newSchema = {};
              if (type === "null") {
                schema.nullable = true;
              } else {
                newSchema.type = type;
                for (let prop of common.arrayProperties) {
                  if (typeof schema.prop !== "undefined") {
                    newSchema[prop] = schema[prop];
                    delete schema[prop];
                  }
                }
              }
              if (newSchema.type) {
                schema.oneOf.push(newSchema);
              }
            }
            delete schema.type;
            if (schema.oneOf.length === 0) {
              delete schema.oneOf;
            } else if (schema.oneOf.length < 2) {
              schema.type = schema.oneOf[0].type;
              if (Object.keys(schema.oneOf[0]).length > 1) {
                throwOrWarn("Lost properties from oneOf", schema, options);
              }
              delete schema.oneOf;
            }
          }
          if (schema.type && Array.isArray(schema.type) && schema.type.length === 1) {
            schema.type = schema.type[0];
          }
        } else {
          throwError("(Patchable) schema type must not be an array", options);
        }
      }
      if (schema.type && schema.type === "null") {
        delete schema.type;
        schema.nullable = true;
      }
      if (schema.type === "array" && !schema.items) {
        schema.items = {};
      }
      if (schema.type === "file") {
        schema.type = "string";
        schema.format = "binary";
      }
      if (typeof schema.required === "boolean") {
        if (schema.required && schema.name) {
          if (typeof parent.required === "undefined") {
            parent.required = [];
          }
          if (Array.isArray(parent.required)) parent.required.push(schema.name);
        }
        delete schema.required;
      }
      if (schema.xml && typeof schema.xml.namespace === "string") {
        if (!schema.xml.namespace) delete schema.xml.namespace;
      }
      if (typeof schema.allowEmptyValue !== "undefined") {
        options.patches++;
        delete schema.allowEmptyValue;
      }
    }
    function fixUpSubSchemaExtensions(schema, parent) {
      if (schema["x-required"] && Array.isArray(schema["x-required"])) {
        if (!schema.required) schema.required = [];
        schema.required = schema.required.concat(schema["x-required"]);
        delete schema["x-required"];
      }
      if (schema["x-anyOf"]) {
        schema.anyOf = schema["x-anyOf"];
        delete schema["x-anyOf"];
      }
      if (schema["x-oneOf"]) {
        schema.oneOf = schema["x-oneOf"];
        delete schema["x-oneOf"];
      }
      if (schema["x-not"]) {
        schema.not = schema["x-not"];
        delete schema["x-not"];
      }
      if (typeof schema["x-nullable"] === "boolean") {
        schema.nullable = schema["x-nullable"];
        delete schema["x-nullable"];
      }
      if (typeof schema["x-discriminator"] === "object" && typeof schema["x-discriminator"].propertyName === "string") {
        schema.discriminator = schema["x-discriminator"];
        delete schema["x-discriminator"];
        for (let entry in schema.discriminator.mapping) {
          let schemaOrRef = schema.discriminator.mapping[entry];
          if (schemaOrRef.startsWith("#/definitions/")) {
            schema.discriminator.mapping[entry] = schemaOrRef.replace("#/definitions/", "#/components/schemas/");
          }
        }
      }
    }
    function fixUpSchema(schema, options) {
      sw.walkSchema(schema, {}, {}, function(schema2, parent, state) {
        fixUpSubSchemaExtensions(schema2, parent);
        fixUpSubSchema(schema2, parent, options);
      });
    }
    function getMiroComponentName(ref) {
      if (ref.indexOf("#") >= 0) {
        ref = ref.split("#")[1].split("/").pop();
      } else {
        ref = ref.split("/").pop().split(".")[0];
      }
      return encodeURIComponent(common.sanitise(ref));
    }
    function fixupRefs(obj, key, state) {
      let options = state.payload.options;
      if (isRef(obj, key)) {
        if (obj[key].startsWith("#/components/")) {
        } else if (obj[key] === "#/consumes") {
          delete obj[key];
          state.parent[state.pkey] = clone(options.openapi.consumes);
        } else if (obj[key] === "#/produces") {
          delete obj[key];
          state.parent[state.pkey] = clone(options.openapi.produces);
        } else if (obj[key].startsWith("#/definitions/")) {
          let keys = obj[key].replace("#/definitions/", "").split("/");
          const ref = jptr.jpunescape(keys[0]);
          let newKey = componentNames.schemas[decodeURIComponent(ref)];
          if (newKey) {
            keys[0] = newKey;
          } else {
            throwOrWarn("Could not resolve reference " + obj[key], obj, options);
          }
          obj[key] = "#/components/schemas/" + keys.join("/");
        } else if (obj[key].startsWith("#/parameters/")) {
          obj[key] = "#/components/parameters/" + common.sanitise(obj[key].replace("#/parameters/", ""));
        } else if (obj[key].startsWith("#/responses/")) {
          obj[key] = "#/components/responses/" + common.sanitise(obj[key].replace("#/responses/", ""));
        } else if (obj[key].startsWith("#")) {
          let target = clone(jptr.jptr(options.openapi, obj[key]));
          if (target === false) throwOrWarn("direct $ref not found " + obj[key], obj, options);
          else if (options.refmap[obj[key]]) {
            obj[key] = options.refmap[obj[key]];
          } else {
            let oldRef = obj[key];
            oldRef = oldRef.replace("/properties/headers/", "");
            oldRef = oldRef.replace("/properties/responses/", "");
            oldRef = oldRef.replace("/properties/parameters/", "");
            oldRef = oldRef.replace("/properties/schemas/", "");
            let type = "schemas";
            let schemaIndex = oldRef.lastIndexOf("/schema");
            type = oldRef.indexOf("/headers/") > schemaIndex ? "headers" : oldRef.indexOf("/responses/") > schemaIndex ? "responses" : oldRef.indexOf("/example") > schemaIndex ? "examples" : oldRef.indexOf("/x-") > schemaIndex ? "extensions" : oldRef.indexOf("/parameters/") > schemaIndex ? "parameters" : "schemas";
            if (type === "schemas") {
              fixUpSchema(target, options);
            }
            if (type !== "responses" && type !== "extensions") {
              let prefix = type.substr(0, type.length - 1);
              if (prefix === "parameter" && target.name && target.name === common.sanitise(target.name)) {
                prefix = encodeURIComponent(target.name);
              }
              let suffix = 1;
              if (obj["x-miro"]) {
                prefix = getMiroComponentName(obj["x-miro"]);
                suffix = "";
              }
              while (jptr.jptr(options.openapi, "#/components/" + type + "/" + prefix + suffix)) {
                suffix = suffix === "" ? 2 : ++suffix;
              }
              let newRef = "#/components/" + type + "/" + prefix + suffix;
              let refSuffix = "";
              if (type === "examples") {
                target = { value: target };
                refSuffix = "/value";
              }
              jptr.jptr(options.openapi, newRef, target);
              options.refmap[obj[key]] = newRef + refSuffix;
              obj[key] = newRef + refSuffix;
            }
          }
        }
        delete obj["x-miro"];
        if (Object.keys(obj).length > 1) {
          const tmpRef = obj[key];
          const inSchema = state.path.indexOf("/schema") >= 0;
          if (options.refSiblings === "preserve") {
          } else if (inSchema && options.refSiblings === "allOf") {
            delete obj.$ref;
            state.parent[state.pkey] = { allOf: [{ $ref: tmpRef }, obj] };
          } else {
            state.parent[state.pkey] = { $ref: tmpRef };
          }
        }
      }
      if (key === "x-ms-odata" && typeof obj[key] === "string" && obj[key].startsWith("#/")) {
        let keys = obj[key].replace("#/definitions/", "").replace("#/components/schemas/", "").split("/");
        let newKey = componentNames.schemas[decodeURIComponent(keys[0])];
        if (newKey) {
          keys[0] = newKey;
        } else {
          throwOrWarn("Could not resolve reference " + obj[key], obj, options);
        }
        obj[key] = "#/components/schemas/" + keys.join("/");
      }
    }
    function dedupeRefs(openapi, options) {
      for (let ref in options.refmap) {
        jptr.jptr(openapi, ref, { $ref: options.refmap[ref] });
      }
    }
    function processSecurity(securityObject) {
      for (let s in securityObject) {
        for (let k in securityObject[s]) {
          let sname = common.sanitise(k);
          if (k !== sname) {
            securityObject[s][sname] = securityObject[s][k];
            delete securityObject[s][k];
          }
        }
      }
    }
    function processSecurityScheme(scheme, options) {
      if (scheme.type === "basic") {
        scheme.type = "http";
        scheme.scheme = "basic";
      }
      if (scheme.type === "oauth2") {
        let flow = {};
        let flowName = scheme.flow;
        if (scheme.flow === "application") flowName = "clientCredentials";
        if (scheme.flow === "accessCode") flowName = "authorizationCode";
        if (typeof scheme.authorizationUrl !== "undefined") flow.authorizationUrl = scheme.authorizationUrl.split("?")[0].trim() || "/";
        if (typeof scheme.tokenUrl === "string") flow.tokenUrl = scheme.tokenUrl.split("?")[0].trim() || "/";
        flow.scopes = scheme.scopes || {};
        scheme.flows = {};
        scheme.flows[flowName] = flow;
        delete scheme.flow;
        delete scheme.authorizationUrl;
        delete scheme.tokenUrl;
        delete scheme.scopes;
        if (typeof scheme.name !== "undefined") {
          if (options.patch) {
            options.patches++;
            delete scheme.name;
          } else {
            throwError("(Patchable) oauth2 securitySchemes should not have name property", options);
          }
        }
      }
    }
    function keepParameters(value) {
      return value && !value["x-s2o-delete"];
    }
    function processHeader(header, options) {
      if (header.$ref) {
        header.$ref = header.$ref.replace("#/responses/", "#/components/responses/");
      } else {
        if (header.type && !header.schema) {
          header.schema = {};
        }
        if (header.type) header.schema.type = header.type;
        if (header.items && header.items.type !== "array") {
          if (header.items.collectionFormat !== header.collectionFormat) {
            throwOrWarn("Nested collectionFormats are not supported", header, options);
          }
          delete header.items.collectionFormat;
        }
        if (header.type === "array") {
          if (header.collectionFormat === "ssv") {
            throwOrWarn("collectionFormat:ssv is no longer supported for headers", header, options);
          } else if (header.collectionFormat === "pipes") {
            throwOrWarn("collectionFormat:pipes is no longer supported for headers", header, options);
          } else if (header.collectionFormat === "multi") {
            header.explode = true;
          } else if (header.collectionFormat === "tsv") {
            throwOrWarn("collectionFormat:tsv is no longer supported", header, options);
            header["x-collectionFormat"] = "tsv";
          } else {
            header.style = "simple";
          }
          delete header.collectionFormat;
        } else if (header.collectionFormat) {
          if (options.patch) {
            options.patches++;
            delete header.collectionFormat;
          } else {
            throwError("(Patchable) collectionFormat is only applicable to header.type array", options);
          }
        }
        delete header.type;
        for (let prop of common.parameterTypeProperties) {
          if (typeof header[prop] !== "undefined") {
            header.schema[prop] = header[prop];
            delete header[prop];
          }
        }
        for (let prop of common.arrayProperties) {
          if (typeof header[prop] !== "undefined") {
            header.schema[prop] = header[prop];
            delete header[prop];
          }
        }
      }
    }
    function fixParamRef(param, options) {
      if (param.$ref.indexOf("#/parameters/") >= 0) {
        let refComponents = param.$ref.split("#/parameters/");
        param.$ref = refComponents[0] + "#/components/parameters/" + common.sanitise(refComponents[1]);
      }
      if (param.$ref.indexOf("#/definitions/") >= 0) {
        throwOrWarn("Definition used as parameter", param, options);
      }
    }
    function attachRequestBody(op, options) {
      let newOp = {};
      for (let key of Object.keys(op)) {
        newOp[key] = op[key];
        if (key === "parameters") {
          newOp.requestBody = {};
          if (options.rbname) newOp[options.rbname] = "";
        }
      }
      newOp.requestBody = {};
      return newOp;
    }
    function processParameter(param, op, path, method, index, openapi, options) {
      let result = {};
      let singularRequestBody = true;
      let originalType;
      if (op && op.consumes && typeof op.consumes === "string") {
        if (options.patch) {
          options.patches++;
          op.consumes = [op.consumes];
        } else {
          return throwError("(Patchable) operation.consumes must be an array", options);
        }
      }
      if (!Array.isArray(openapi.consumes)) delete openapi.consumes;
      let consumes = ((op ? op.consumes : null) || (openapi.consumes || [])).filter(common.uniqueOnly);
      if (param && param.$ref && typeof param.$ref === "string") {
        fixParamRef(param, options);
        let ptr = decodeURIComponent(param.$ref.replace("#/components/parameters/", ""));
        let rbody = false;
        let target = openapi.components.parameters[ptr];
        if ((!target || target["x-s2o-delete"]) && param.$ref.startsWith("#/")) {
          param["x-s2o-delete"] = true;
          rbody = true;
        }
        if (rbody) {
          let ref = param.$ref;
          let newParam = resolveInternal(openapi, param.$ref);
          if (!newParam && ref.startsWith("#/")) {
            throwOrWarn("Could not resolve reference " + ref, param, options);
          } else {
            if (newParam) param = newParam;
          }
        }
      }
      if (param && (param.name || param.in)) {
        if (typeof param["x-deprecated"] === "boolean") {
          param.deprecated = param["x-deprecated"];
          delete param["x-deprecated"];
        }
        if (typeof param["x-example"] !== "undefined") {
          param.example = param["x-example"];
          delete param["x-example"];
        }
        if (param.in !== "body" && !param.type) {
          if (options.patch) {
            options.patches++;
            param.type = "string";
          } else {
            throwError("(Patchable) parameter.type is mandatory for non-body parameters", options);
          }
        }
        if (param.type && typeof param.type === "object" && param.type.$ref) {
          param.type = resolveInternal(openapi, param.type.$ref);
        }
        if (param.type === "file") {
          param["x-s2o-originalType"] = param.type;
          originalType = param.type;
        }
        if (param.description && typeof param.description === "object" && param.description.$ref) {
          param.description = resolveInternal(openapi, param.description.$ref);
        }
        if (param.description === null) delete param.description;
        let oldCollectionFormat = param.collectionFormat;
        if (param.type === "array" && !oldCollectionFormat) {
          oldCollectionFormat = "csv";
        }
        if (oldCollectionFormat) {
          if (param.type !== "array") {
            if (options.patch) {
              options.patches++;
              delete param.collectionFormat;
            } else {
              throwError("(Patchable) collectionFormat is only applicable to param.type array", options);
            }
          }
          if (oldCollectionFormat === "csv" && (param.in === "query" || param.in === "cookie")) {
            param.style = "form";
            param.explode = false;
          }
          if (oldCollectionFormat === "csv" && (param.in === "path" || param.in === "header")) {
            param.style = "simple";
          }
          if (oldCollectionFormat === "ssv") {
            if (param.in === "query") {
              param.style = "spaceDelimited";
            } else {
              throwOrWarn("collectionFormat:ssv is no longer supported except for in:query parameters", param, options);
            }
          }
          if (oldCollectionFormat === "pipes") {
            if (param.in === "query") {
              param.style = "pipeDelimited";
            } else {
              throwOrWarn("collectionFormat:pipes is no longer supported except for in:query parameters", param, options);
            }
          }
          if (oldCollectionFormat === "multi") {
            param.explode = true;
          }
          if (oldCollectionFormat === "tsv") {
            throwOrWarn("collectionFormat:tsv is no longer supported", param, options);
            param["x-collectionFormat"] = "tsv";
          }
          delete param.collectionFormat;
        }
        if (param.type && param.type !== "body" && param.in !== "formData") {
          if (param.items && param.schema) {
            throwOrWarn("parameter has array,items and schema", param, options);
          } else {
            if (param.schema) options.patches++;
            if (!param.schema || typeof param.schema !== "object") param.schema = {};
            param.schema.type = param.type;
            if (param.items) {
              param.schema.items = param.items;
              delete param.items;
              recurse(param.schema.items, null, function(obj, key, state) {
                if (key === "collectionFormat" && typeof obj[key] === "string") {
                  if (oldCollectionFormat && obj[key] !== oldCollectionFormat) {
                    throwOrWarn("Nested collectionFormats are not supported", param, options);
                  }
                  delete obj[key];
                }
              });
            }
            for (let prop of common.parameterTypeProperties) {
              if (typeof param[prop] !== "undefined") param.schema[prop] = param[prop];
              delete param[prop];
            }
          }
        }
        if (param.schema) {
          fixUpSchema(param.schema, options);
        }
        if (param["x-ms-skip-url-encoding"]) {
          if (param.in === "query") {
            param.allowReserved = true;
            delete param["x-ms-skip-url-encoding"];
          }
        }
      }
      if (param && param.in === "formData") {
        singularRequestBody = false;
        result.content = {};
        let contentType = "application/x-www-form-urlencoded";
        if (consumes.length && consumes.indexOf("multipart/form-data") >= 0) {
          contentType = "multipart/form-data";
        }
        result.content[contentType] = {};
        if (param.schema) {
          result.content[contentType].schema = param.schema;
          if (param.schema.$ref) {
            result["x-s2o-name"] = decodeURIComponent(param.schema.$ref.replace("#/components/schemas/", ""));
          }
        } else {
          result.content[contentType].schema = {};
          result.content[contentType].schema.type = "object";
          result.content[contentType].schema.properties = {};
          result.content[contentType].schema.properties[param.name] = {};
          let schema = result.content[contentType].schema;
          let target = result.content[contentType].schema.properties[param.name];
          if (param.description) target.description = param.description;
          if (param.example) target.example = param.example;
          if (param.type) target.type = param.type;
          for (let prop of common.parameterTypeProperties) {
            if (typeof param[prop] !== "undefined") target[prop] = param[prop];
          }
          if (param.required === true) {
            if (!schema.required) schema.required = [];
            schema.required.push(param.name);
            result.required = true;
          }
          if (typeof param.default !== "undefined") target.default = param.default;
          if (target.properties) target.properties = param.properties;
          if (param.allOf) target.allOf = param.allOf;
          if (param.type === "array" && param.items) {
            target.items = param.items;
            if (target.items.collectionFormat) delete target.items.collectionFormat;
          }
          if (originalType === "file" || param["x-s2o-originalType"] === "file") {
            target.type = "string";
            target.format = "binary";
          }
          copyExtensions(param, target);
        }
      } else if (param && param.type === "file") {
        if (param.required) result.required = param.required;
        result.content = {};
        result.content["application/octet-stream"] = {};
        result.content["application/octet-stream"].schema = {};
        result.content["application/octet-stream"].schema.type = "string";
        result.content["application/octet-stream"].schema.format = "binary";
        copyExtensions(param, result);
      }
      if (param && param.in === "body") {
        result.content = {};
        if (param.name) result["x-s2o-name"] = (op && op.operationId ? common.sanitiseAll(op.operationId) : "") + ("_" + param.name).toCamelCase();
        if (param.description) result.description = param.description;
        if (param.required) result.required = param.required;
        if (op && options.rbname && param.name) {
          op[options.rbname] = param.name;
        }
        if (param.schema && param.schema.$ref) {
          result["x-s2o-name"] = decodeURIComponent(param.schema.$ref.replace("#/components/schemas/", ""));
        } else if (param.schema && param.schema.type === "array" && param.schema.items && param.schema.items.$ref) {
          result["x-s2o-name"] = decodeURIComponent(param.schema.items.$ref.replace("#/components/schemas/", "")) + "Array";
        }
        if (!consumes.length) {
          consumes.push("application/json");
        }
        for (let mimetype of consumes) {
          result.content[mimetype] = {};
          result.content[mimetype].schema = clone(param.schema || {});
          fixUpSchema(result.content[mimetype].schema, options);
        }
        copyExtensions(param, result);
      }
      if (Object.keys(result).length > 0) {
        param["x-s2o-delete"] = true;
        if (op) {
          if (op.requestBody && singularRequestBody) {
            op.requestBody["x-s2o-overloaded"] = true;
            let opId = op.operationId || index;
            throwOrWarn("Operation " + opId + " has multiple requestBodies", op, options);
          } else {
            if (!op.requestBody) {
              op = path[method] = attachRequestBody(op, options);
            }
            if (op.requestBody.content && op.requestBody.content["multipart/form-data"] && op.requestBody.content["multipart/form-data"].schema && op.requestBody.content["multipart/form-data"].schema.properties && result.content["multipart/form-data"] && result.content["multipart/form-data"].schema && result.content["multipart/form-data"].schema.properties) {
              op.requestBody.content["multipart/form-data"].schema.properties = Object.assign(op.requestBody.content["multipart/form-data"].schema.properties, result.content["multipart/form-data"].schema.properties);
              op.requestBody.content["multipart/form-data"].schema.required = (op.requestBody.content["multipart/form-data"].schema.required || []).concat(result.content["multipart/form-data"].schema.required || []);
              if (!op.requestBody.content["multipart/form-data"].schema.required.length) {
                delete op.requestBody.content["multipart/form-data"].schema.required;
              }
            } else if (op.requestBody.content && op.requestBody.content["application/x-www-form-urlencoded"] && op.requestBody.content["application/x-www-form-urlencoded"].schema && op.requestBody.content["application/x-www-form-urlencoded"].schema.properties && result.content["application/x-www-form-urlencoded"] && result.content["application/x-www-form-urlencoded"].schema && result.content["application/x-www-form-urlencoded"].schema.properties) {
              op.requestBody.content["application/x-www-form-urlencoded"].schema.properties = Object.assign(op.requestBody.content["application/x-www-form-urlencoded"].schema.properties, result.content["application/x-www-form-urlencoded"].schema.properties);
              op.requestBody.content["application/x-www-form-urlencoded"].schema.required = (op.requestBody.content["application/x-www-form-urlencoded"].schema.required || []).concat(result.content["application/x-www-form-urlencoded"].schema.required || []);
              if (!op.requestBody.content["application/x-www-form-urlencoded"].schema.required.length) {
                delete op.requestBody.content["application/x-www-form-urlencoded"].schema.required;
              }
            } else {
              op.requestBody = Object.assign(op.requestBody, result);
              if (!op.requestBody["x-s2o-name"]) {
                if (op.requestBody.schema && op.requestBody.schema.$ref) {
                  op.requestBody["x-s2o-name"] = decodeURIComponent(op.requestBody.schema.$ref.replace("#/components/schemas/", "")).split("/").join("");
                } else if (op.operationId) {
                  op.requestBody["x-s2o-name"] = common.sanitiseAll(op.operationId);
                }
              }
            }
          }
        }
      }
      if (param && !param["x-s2o-delete"]) {
        delete param.type;
        for (let prop of common.parameterTypeProperties) {
          delete param[prop];
        }
        if (param.in === "path" && (typeof param.required === "undefined" || param.required !== true)) {
          if (options.patch) {
            options.patches++;
            param.required = true;
          } else {
            throwError("(Patchable) path parameters must be required:true [" + param.name + " in " + index + "]", options);
          }
        }
      }
      return op;
    }
    function copyExtensions(src, tgt) {
      for (let prop in src) {
        if (prop.startsWith("x-") && !prop.startsWith("x-s2o")) {
          tgt[prop] = src[prop];
        }
      }
    }
    function processResponse(response, name, op, openapi, options) {
      if (!response) return false;
      if (response.$ref && typeof response.$ref === "string") {
        if (response.$ref.indexOf("#/definitions/") >= 0) {
          throwOrWarn("definition used as response: " + response.$ref, response, options);
        } else {
          if (response.$ref.startsWith("#/responses/")) {
            response.$ref = "#/components/responses/" + common.sanitise(decodeURIComponent(response.$ref.replace("#/responses/", "")));
          }
        }
      } else {
        if (typeof response.description === "undefined" || response.description === null || response.description === "" && options.patch) {
          if (options.patch) {
            if (typeof response === "object" && !Array.isArray(response)) {
              options.patches++;
              response.description = statusCodes[response] || "";
            }
          } else {
            throwError("(Patchable) response.description is mandatory", options);
          }
        }
        if (typeof response.schema !== "undefined") {
          fixUpSchema(response.schema, options);
          if (response.schema.$ref && typeof response.schema.$ref === "string" && response.schema.$ref.startsWith("#/responses/")) {
            response.schema.$ref = "#/components/responses/" + common.sanitise(decodeURIComponent(response.schema.$ref.replace("#/responses/", "")));
          }
          if (op && op.produces && typeof op.produces === "string") {
            if (options.patch) {
              options.patches++;
              op.produces = [op.produces];
            } else {
              return throwError("(Patchable) operation.produces must be an array", options);
            }
          }
          if (openapi.produces && !Array.isArray(openapi.produces)) delete openapi.produces;
          let produces = ((op ? op.produces : null) || (openapi.produces || [])).filter(common.uniqueOnly);
          if (!produces.length) produces.push("*/*");
          response.content = {};
          for (let mimetype of produces) {
            response.content[mimetype] = {};
            response.content[mimetype].schema = clone(response.schema);
            if (response.examples && response.examples[mimetype]) {
              let example = {};
              example.value = response.examples[mimetype];
              response.content[mimetype].examples = {};
              response.content[mimetype].examples.response = example;
              delete response.examples[mimetype];
            }
            if (response.content[mimetype].schema.type === "file") {
              response.content[mimetype].schema = { type: "string", format: "binary" };
            }
          }
          delete response.schema;
        }
        for (let mimetype in response.examples) {
          if (!response.content) response.content = {};
          if (!response.content[mimetype]) response.content[mimetype] = {};
          response.content[mimetype].examples = {};
          response.content[mimetype].examples.response = {};
          response.content[mimetype].examples.response.value = response.examples[mimetype];
        }
        delete response.examples;
        if (response.headers) {
          for (let h in response.headers) {
            if (h.toLowerCase() === "status code") {
              if (options.patch) {
                options.patches++;
                delete response.headers[h];
              } else {
                throwError('(Patchable) "Status Code" is not a valid header', options);
              }
            } else {
              processHeader(response.headers[h], options);
            }
          }
        }
      }
    }
    function processPaths(container, containerName, options, requestBodyCache, openapi) {
      for (let p in container) {
        let path = container[p];
        if (path && path["x-trace"] && typeof path["x-trace"] === "object") {
          path.trace = path["x-trace"];
          delete path["x-trace"];
        }
        if (path && path["x-summary"] && typeof path["x-summary"] === "string") {
          path.summary = path["x-summary"];
          delete path["x-summary"];
        }
        if (path && path["x-description"] && typeof path["x-description"] === "string") {
          path.description = path["x-description"];
          delete path["x-description"];
        }
        if (path && path["x-servers"] && Array.isArray(path["x-servers"])) {
          path.servers = path["x-servers"];
          delete path["x-servers"];
        }
        for (let method in path) {
          if (common.httpMethods.indexOf(method) >= 0 || method === "x-amazon-apigateway-any-method") {
            let op = path[method];
            if (op && op.parameters && Array.isArray(op.parameters)) {
              if (path.parameters) {
                for (let param of path.parameters) {
                  if (typeof param.$ref === "string") {
                    fixParamRef(param, options);
                    param = resolveInternal(openapi, param.$ref);
                  }
                  let match = op.parameters.find(function(e, i, a) {
                    return e.name === param.name && e.in === param.in;
                  });
                  if (!match && (param.in === "formData" || param.in === "body" || param.type === "file")) {
                    op = processParameter(param, op, path, method, p, openapi, options);
                    if (options.rbname && op[options.rbname] === "") {
                      delete op[options.rbname];
                    }
                  }
                }
              }
              for (let param of op.parameters) {
                op = processParameter(param, op, path, method, method + ":" + p, openapi, options);
              }
              if (options.rbname && op[options.rbname] === "") {
                delete op[options.rbname];
              }
              if (!options.debug) {
                if (op.parameters) op.parameters = op.parameters.filter(keepParameters);
              }
            }
            if (op && op.security) processSecurity(op.security);
            if (typeof op === "object") {
              if (!op.responses) {
                let defaultResp = {};
                defaultResp.description = "Default response";
                op.responses = { default: defaultResp };
              }
              for (let r in op.responses) {
                let response = op.responses[r];
                processResponse(response, r, op, openapi, options);
              }
            }
            if (op && op["x-servers"] && Array.isArray(op["x-servers"])) {
              op.servers = op["x-servers"];
              delete op["x-servers"];
            } else if (op && op.schemes && op.schemes.length) {
              for (let scheme of op.schemes) {
                if (!openapi.schemes || openapi.schemes.indexOf(scheme) < 0) {
                  if (!op.servers) {
                    op.servers = [];
                  }
                  if (Array.isArray(openapi.servers)) {
                    for (let server of openapi.servers) {
                      let newServer = clone(server);
                      let serverUrl = url.parse(newServer.url);
                      serverUrl.protocol = scheme;
                      newServer.url = serverUrl.format();
                      op.servers.push(newServer);
                    }
                  }
                }
              }
            }
            if (options.debug) {
              op["x-s2o-consumes"] = op.consumes || [];
              op["x-s2o-produces"] = op.produces || [];
            }
            if (op) {
              delete op.consumes;
              delete op.produces;
              delete op.schemes;
              if (op["x-ms-examples"]) {
                for (let e in op["x-ms-examples"]) {
                  let example = op["x-ms-examples"][e];
                  let se = common.sanitiseAll(e);
                  if (example.parameters) {
                    for (let p2 in example.parameters) {
                      let value = example.parameters[p2];
                      for (let param of (op.parameters || []).concat(path.parameters || [])) {
                        if (param.$ref) {
                          param = jptr.jptr(openapi, param.$ref);
                        }
                        if (param.name === p2 && !param.example) {
                          if (!param.examples) {
                            param.examples = {};
                          }
                          param.examples[e] = { value };
                        }
                      }
                    }
                  }
                  if (example.responses) {
                    for (let r in example.responses) {
                      if (example.responses[r].headers) {
                        for (let h in example.responses[r].headers) {
                          let value = example.responses[r].headers[h];
                          for (let rh in op.responses[r].headers) {
                            if (rh === h) {
                              let header = op.responses[r].headers[rh];
                              header.example = value;
                            }
                          }
                        }
                      }
                      if (example.responses[r].body) {
                        openapi.components.examples[se] = { value: clone(example.responses[r].body) };
                        if (op.responses[r] && op.responses[r].content) {
                          for (let ct in op.responses[r].content) {
                            let contentType = op.responses[r].content[ct];
                            if (!contentType.examples) {
                              contentType.examples = {};
                            }
                            contentType.examples[e] = { $ref: "#/components/examples/" + se };
                          }
                        }
                      }
                    }
                  }
                }
                delete op["x-ms-examples"];
              }
              if (op.parameters && op.parameters.length === 0) delete op.parameters;
              if (op.requestBody) {
                let effectiveOperationId = op.operationId ? common.sanitiseAll(op.operationId) : common.sanitiseAll(method + p).toCamelCase();
                let rbName = common.sanitise(op.requestBody["x-s2o-name"] || effectiveOperationId || "");
                delete op.requestBody["x-s2o-name"];
                let rbStr = JSON.stringify(op.requestBody);
                let rbHash = common.hash(rbStr);
                if (!requestBodyCache[rbHash]) {
                  let entry = {};
                  entry.name = rbName;
                  entry.body = op.requestBody;
                  entry.refs = [];
                  requestBodyCache[rbHash] = entry;
                }
                let ptr = "#/" + containerName + "/" + encodeURIComponent(jptr.jpescape(p)) + "/" + method + "/requestBody";
                requestBodyCache[rbHash].refs.push(ptr);
              }
            }
          }
        }
        if (path && path.parameters) {
          for (let p2 in path.parameters) {
            let param = path.parameters[p2];
            processParameter(param, null, path, null, p, openapi, options);
          }
          if (!options.debug && Array.isArray(path.parameters)) {
            path.parameters = path.parameters.filter(keepParameters);
          }
        }
      }
    }
    function main2(openapi, options) {
      let requestBodyCache = {};
      componentNames = { schemas: {} };
      if (openapi.security) processSecurity(openapi.security);
      for (let s in openapi.components.securitySchemes) {
        let sname = common.sanitise(s);
        if (s !== sname) {
          if (openapi.components.securitySchemes[sname]) {
            throwError("Duplicate sanitised securityScheme name " + sname, options);
          }
          openapi.components.securitySchemes[sname] = openapi.components.securitySchemes[s];
          delete openapi.components.securitySchemes[s];
        }
        processSecurityScheme(openapi.components.securitySchemes[sname], options);
      }
      for (let s in openapi.components.schemas) {
        let sname = common.sanitiseAll(s);
        let suffix = "";
        if (s !== sname) {
          while (openapi.components.schemas[sname + suffix]) {
            suffix = suffix ? ++suffix : 2;
          }
          openapi.components.schemas[sname + suffix] = openapi.components.schemas[s];
          delete openapi.components.schemas[s];
        }
        componentNames.schemas[s] = sname + suffix;
        fixUpSchema(openapi.components.schemas[sname + suffix], options);
      }
      options.refmap = {};
      recurse(openapi, { payload: { options } }, fixupRefs);
      dedupeRefs(openapi, options);
      for (let p in openapi.components.parameters) {
        let sname = common.sanitise(p);
        if (p !== sname) {
          if (openapi.components.parameters[sname]) {
            throwError("Duplicate sanitised parameter name " + sname, options);
          }
          openapi.components.parameters[sname] = openapi.components.parameters[p];
          delete openapi.components.parameters[p];
        }
        let param = openapi.components.parameters[sname];
        processParameter(param, null, null, null, sname, openapi, options);
      }
      for (let r in openapi.components.responses) {
        let sname = common.sanitise(r);
        if (r !== sname) {
          if (openapi.components.responses[sname]) {
            throwError("Duplicate sanitised response name " + sname, options);
          }
          openapi.components.responses[sname] = openapi.components.responses[r];
          delete openapi.components.responses[r];
        }
        let response = openapi.components.responses[sname];
        processResponse(response, sname, null, openapi, options);
        if (response.headers) {
          for (let h in response.headers) {
            if (h.toLowerCase() === "status code") {
              if (options.patch) {
                options.patches++;
                delete response.headers[h];
              } else {
                throwError('(Patchable) "Status Code" is not a valid header', options);
              }
            } else {
              processHeader(response.headers[h], options);
            }
          }
        }
      }
      for (let r in openapi.components.requestBodies) {
        let rb = openapi.components.requestBodies[r];
        let rbStr = JSON.stringify(rb);
        let rbHash = common.hash(rbStr);
        let entry = {};
        entry.name = r;
        entry.body = rb;
        entry.refs = [];
        requestBodyCache[rbHash] = entry;
      }
      processPaths(openapi.paths, "paths", options, requestBodyCache, openapi);
      if (openapi["x-ms-paths"]) {
        processPaths(openapi["x-ms-paths"], "x-ms-paths", options, requestBodyCache, openapi);
      }
      if (!options.debug) {
        for (let p in openapi.components.parameters) {
          let param = openapi.components.parameters[p];
          if (param["x-s2o-delete"]) {
            delete openapi.components.parameters[p];
          }
        }
      }
      if (options.debug) {
        openapi["x-s2o-consumes"] = openapi.consumes || [];
        openapi["x-s2o-produces"] = openapi.produces || [];
      }
      delete openapi.consumes;
      delete openapi.produces;
      delete openapi.schemes;
      let rbNamesGenerated = [];
      openapi.components.requestBodies = {};
      if (!options.resolveInternal) {
        let counter = 1;
        for (let e in requestBodyCache) {
          let entry = requestBodyCache[e];
          if (entry.refs.length > 1) {
            let suffix = "";
            if (!entry.name) {
              entry.name = "requestBody";
              suffix = counter++;
            }
            while (rbNamesGenerated.indexOf(entry.name + suffix) >= 0) {
              suffix = suffix ? ++suffix : 2;
            }
            entry.name = entry.name + suffix;
            rbNamesGenerated.push(entry.name);
            openapi.components.requestBodies[entry.name] = clone(entry.body);
            for (let r in entry.refs) {
              let ref = {};
              ref.$ref = "#/components/requestBodies/" + entry.name;
              jptr.jptr(openapi, entry.refs[r], ref);
            }
          }
        }
      }
      if (openapi.components.responses && Object.keys(openapi.components.responses).length === 0) {
        delete openapi.components.responses;
      }
      if (openapi.components.parameters && Object.keys(openapi.components.parameters).length === 0) {
        delete openapi.components.parameters;
      }
      if (openapi.components.examples && Object.keys(openapi.components.examples).length === 0) {
        delete openapi.components.examples;
      }
      if (openapi.components.requestBodies && Object.keys(openapi.components.requestBodies).length === 0) {
        delete openapi.components.requestBodies;
      }
      if (openapi.components.securitySchemes && Object.keys(openapi.components.securitySchemes).length === 0) {
        delete openapi.components.securitySchemes;
      }
      if (openapi.components.headers && Object.keys(openapi.components.headers).length === 0) {
        delete openapi.components.headers;
      }
      if (openapi.components.schemas && Object.keys(openapi.components.schemas).length === 0) {
        delete openapi.components.schemas;
      }
      if (openapi.components && Object.keys(openapi.components).length === 0) {
        delete openapi.components;
      }
      return openapi;
    }
    function extractServerParameters(server) {
      if (!server || !server.url || typeof server.url !== "string") return server;
      server.url = server.url.split("{{").join("{");
      server.url = server.url.split("}}").join("}");
      server.url.replace(/\{(.+?)\}/g, function(match, group1) {
        if (!server.variables) {
          server.variables = {};
        }
        server.variables[group1] = { default: "unknown" };
      });
      return server;
    }
    function fixInfo(openapi, options, reject) {
      if (typeof openapi.info === "undefined" || openapi.info === null) {
        if (options.patch) {
          options.patches++;
          openapi.info = { version: "", title: "" };
        } else {
          return reject(new S2OError("(Patchable) info object is mandatory"));
        }
      }
      if (typeof openapi.info !== "object" || Array.isArray(openapi.info)) {
        return reject(new S2OError("info must be an object"));
      }
      if (typeof openapi.info.title === "undefined" || openapi.info.title === null) {
        if (options.patch) {
          options.patches++;
          openapi.info.title = "";
        } else {
          return reject(new S2OError("(Patchable) info.title cannot be null"));
        }
      }
      if (typeof openapi.info.version === "undefined" || openapi.info.version === null) {
        if (options.patch) {
          options.patches++;
          openapi.info.version = "";
        } else {
          return reject(new S2OError("(Patchable) info.version cannot be null"));
        }
      }
      if (typeof openapi.info.version !== "string") {
        if (options.patch) {
          options.patches++;
          openapi.info.version = openapi.info.version.toString();
        } else {
          return reject(new S2OError("(Patchable) info.version must be a string"));
        }
      }
      if (typeof openapi.info.logo !== "undefined") {
        if (options.patch) {
          options.patches++;
          openapi.info["x-logo"] = openapi.info.logo;
          delete openapi.info.logo;
        } else return reject(new S2OError("(Patchable) info should not have logo property"));
      }
      if (typeof openapi.info.termsOfService !== "undefined") {
        if (openapi.info.termsOfService === null) {
          if (options.patch) {
            options.patches++;
            openapi.info.termsOfService = "";
          } else {
            return reject(new S2OError("(Patchable) info.termsOfService cannot be null"));
          }
        }
        try {
          let u = new URL(openapi.info.termsOfService);
        } catch (ex) {
          if (options.patch) {
            options.patches++;
            delete openapi.info.termsOfService;
          } else return reject(new S2OError("(Patchable) info.termsOfService must be a URL"));
        }
      }
    }
    function fixPaths(openapi, options, reject) {
      if (typeof openapi.paths === "undefined") {
        if (options.patch) {
          options.patches++;
          openapi.paths = {};
        } else {
          return reject(new S2OError("(Patchable) paths object is mandatory"));
        }
      }
    }
    function detectObjectReferences(obj, options) {
      const seen = /* @__PURE__ */ new WeakSet();
      recurse(obj, { identityDetection: true }, function(obj2, key, state) {
        if (typeof obj2[key] === "object" && obj2[key] !== null) {
          if (seen.has(obj2[key])) {
            if (options.anchors) {
              obj2[key] = clone(obj2[key]);
            } else {
              throwError("YAML anchor or merge key at " + state.path, options);
            }
          } else {
            seen.add(obj2[key]);
          }
        }
      });
    }
    function convertObj(swagger, options, callback) {
      return maybe(callback, new Promise(function(resolve, reject) {
        if (!swagger) swagger = {};
        options.original = swagger;
        if (!options.text) options.text = yaml.stringify(swagger);
        options.externals = [];
        options.externalRefs = {};
        options.rewriteRefs = true;
        options.preserveMiro = true;
        options.promise = {};
        options.promise.resolve = resolve;
        options.promise.reject = reject;
        options.patches = 0;
        if (!options.cache) options.cache = {};
        if (options.source) options.cache[options.source] = options.original;
        detectObjectReferences(swagger, options);
        if (swagger.openapi && typeof swagger.openapi === "string" && swagger.openapi.startsWith("3.")) {
          options.openapi = cclone(swagger);
          fixInfo(options.openapi, options, reject);
          fixPaths(options.openapi, options, reject);
          resolver.optionalResolve(options).then(function() {
            if (options.direct) {
              return resolve(options.openapi);
            } else {
              return resolve(options);
            }
          }).catch(function(ex) {
            console.warn(ex);
            reject(ex);
          });
          return;
        }
        if (!swagger.swagger || swagger.swagger != "2.0") {
          return reject(new S2OError("Unsupported swagger/OpenAPI version: " + (swagger.openapi ? swagger.openapi : swagger.swagger)));
        }
        let openapi = options.openapi = {};
        openapi.openapi = typeof options.targetVersion === "string" && options.targetVersion.startsWith("3.") ? options.targetVersion : targetVersion;
        if (options.origin) {
          if (!openapi["x-origin"]) {
            openapi["x-origin"] = [];
          }
          let origin = {};
          origin.url = options.source || options.origin;
          origin.format = "swagger";
          origin.version = swagger.swagger;
          origin.converter = {};
          origin.converter.url = "https://github.com/mermade/oas-kit";
          origin.converter.version = ourVersion;
          openapi["x-origin"].push(origin);
        }
        openapi = Object.assign(openapi, cclone(swagger));
        delete openapi.swagger;
        recurse(openapi, {}, function(obj, key, state) {
          if (obj[key] === null && !key.startsWith("x-") && key !== "default" && state.path.indexOf("/example") < 0) delete obj[key];
        });
        if (swagger.host) {
          for (let s of Array.isArray(swagger.schemes) ? swagger.schemes : [""]) {
            let server = {};
            let basePath = (swagger.basePath || "").replace(/\/$/, "");
            server.url = (s ? s + ":" : "") + "//" + swagger.host + basePath;
            extractServerParameters(server);
            if (!openapi.servers) openapi.servers = [];
            openapi.servers.push(server);
          }
        } else if (swagger.basePath) {
          let server = {};
          server.url = swagger.basePath;
          extractServerParameters(server);
          if (!openapi.servers) openapi.servers = [];
          openapi.servers.push(server);
        }
        delete openapi.host;
        delete openapi.basePath;
        if (openapi["x-servers"] && Array.isArray(openapi["x-servers"])) {
          openapi.servers = openapi["x-servers"];
          delete openapi["x-servers"];
        }
        if (swagger["x-ms-parameterized-host"]) {
          let xMsPHost = swagger["x-ms-parameterized-host"];
          let server = {};
          server.url = xMsPHost.hostTemplate + (swagger.basePath ? swagger.basePath : "");
          server.variables = {};
          const paramNames = server.url.match(/\{\w+\}/g);
          for (let msp in xMsPHost.parameters) {
            let param = xMsPHost.parameters[msp];
            if (param.$ref) {
              param = clone(resolveInternal(openapi, param.$ref));
            }
            if (!msp.startsWith("x-")) {
              delete param.required;
              delete param.type;
              delete param.in;
              if (typeof param.default === "undefined") {
                if (param.enum) {
                  param.default = param.enum[0];
                } else {
                  param.default = "none";
                }
              }
              if (!param.name) {
                param.name = paramNames[msp].replace("{", "").replace("}", "");
              }
              server.variables[param.name] = param;
              delete param.name;
            }
          }
          if (!openapi.servers) openapi.servers = [];
          if (xMsPHost.useSchemePrefix === false) {
            openapi.servers.push(server);
          } else {
            swagger.schemes.forEach((scheme) => {
              openapi.servers.push(
                Object.assign({}, server, { url: scheme + "://" + server.url })
              );
            });
          }
          delete openapi["x-ms-parameterized-host"];
        }
        fixInfo(openapi, options, reject);
        fixPaths(openapi, options, reject);
        if (typeof openapi.consumes === "string") {
          openapi.consumes = [openapi.consumes];
        }
        if (typeof openapi.produces === "string") {
          openapi.produces = [openapi.produces];
        }
        openapi.components = {};
        if (openapi["x-callbacks"]) {
          openapi.components.callbacks = openapi["x-callbacks"];
          delete openapi["x-callbacks"];
        }
        openapi.components.examples = {};
        openapi.components.headers = {};
        if (openapi["x-links"]) {
          openapi.components.links = openapi["x-links"];
          delete openapi["x-links"];
        }
        openapi.components.parameters = openapi.parameters || {};
        openapi.components.responses = openapi.responses || {};
        openapi.components.requestBodies = {};
        openapi.components.securitySchemes = openapi.securityDefinitions || {};
        openapi.components.schemas = openapi.definitions || {};
        delete openapi.definitions;
        delete openapi.responses;
        delete openapi.parameters;
        delete openapi.securityDefinitions;
        resolver.optionalResolve(options).then(function() {
          main2(options.openapi, options);
          if (options.direct) {
            resolve(options.openapi);
          } else {
            resolve(options);
          }
        }).catch(function(ex) {
          console.warn(ex);
          reject(ex);
        });
      }));
    }
    function convertStr(str, options, callback) {
      return maybe(callback, new Promise(function(resolve, reject) {
        let obj = null;
        let error = null;
        try {
          obj = JSON.parse(str);
          options.text = JSON.stringify(obj, null, 2);
        } catch (ex) {
          error = ex;
          try {
            obj = yaml.parse(str, { schema: "core", prettyErrors: true });
            options.sourceYaml = true;
            options.text = str;
          } catch (ex2) {
            error = ex2;
          }
        }
        if (obj) {
          convertObj(obj, options).then((options2) => resolve(options2)).catch((ex) => reject(ex));
        } else {
          reject(new S2OError(error ? error.message : "Could not parse string"));
        }
      }));
    }
    function convertUrl(url2, options, callback) {
      return maybe(callback, new Promise(function(resolve, reject) {
        options.origin = true;
        if (!options.source) {
          options.source = url2;
        }
        if (options.verbose) {
          console.warn("GET " + url2);
        }
        if (!options.fetch) {
          options.fetch = fetch2;
        }
        const fetchOptions = Object.assign({}, options.fetchOptions, { agent: options.agent });
        options.fetch(url2, fetchOptions).then(function(res) {
          if (res.status !== 200) throw new S2OError(`Received status code ${res.status}: ${url2}`);
          return res.text();
        }).then(function(body) {
          convertStr(body, options).then((options2) => resolve(options2)).catch((ex) => reject(ex));
        }).catch(function(err) {
          reject(err);
        });
      }));
    }
    function convertFile(filename, options, callback) {
      return maybe(callback, new Promise(function(resolve, reject) {
        fs.readFile(filename, options.encoding || "utf8", function(err, s) {
          if (err) {
            reject(err);
          } else {
            options.sourceFile = filename;
            convertStr(s, options).then((options2) => resolve(options2)).catch((ex) => reject(ex));
          }
        });
      }));
    }
    function convertStream(readable, options, callback) {
      return maybe(callback, new Promise(function(resolve, reject) {
        let data = "";
        readable.on("data", function(chunk) {
          data += chunk;
        }).on("end", function() {
          convertStr(data, options).then((options2) => resolve(options2)).catch((ex) => reject(ex));
        });
      }));
    }
    module2.exports = {
      S2OError,
      targetVersion,
      convert: convertObj,
      convertObj,
      convertUrl,
      convertStr,
      convertFile,
      convertStream
    };
  }
});

// infra/openapi-sidecar/src/convert.ts
async function loadSwagger2Openapi() {
  const mod = await Promise.resolve().then(() => __toESM(require_swagger2openapi()));
  return mod?.default ?? mod;
}
function deepWalk(value, visit) {
  if (Array.isArray(value)) {
    for (const v of value) deepWalk(v, visit);
    return;
  }
  if (value && typeof value === "object") {
    visit(value);
    for (const v of Object.values(value)) deepWalk(v, visit);
  }
}
function keepGetOnly(spec) {
  const out = structuredClone(spec);
  const paths = out.paths ?? {};
  for (const [path, ops] of Object.entries(paths)) {
    if (!ops || typeof ops !== "object") continue;
    const getOp = ops.get;
    if (!getOp || typeof getOp !== "object") {
      delete paths[path];
      continue;
    }
    paths[path] = { get: getOp };
  }
  out.paths = paths;
  return out;
}
function setGatewayServers(spec) {
  const out = structuredClone(spec);
  out.servers = [{ url: "/" }];
  return out;
}
function ensureOperationIds(spec) {
  const out = structuredClone(spec);
  const used = /* @__PURE__ */ new Set();
  for (const ops of Object.values(out.paths ?? {})) {
    if (!ops || typeof ops !== "object") continue;
    for (const op of Object.values(ops)) {
      if (!op || typeof op !== "object") continue;
      const id = op.operationId;
      if (typeof id === "string" && id.trim()) used.add(id);
    }
  }
  for (const [path, ops] of Object.entries(out.paths ?? {})) {
    if (!ops || typeof ops !== "object") continue;
    for (const [method, op] of Object.entries(ops)) {
      if (!op || typeof op !== "object") continue;
      const existing = op.operationId;
      if (typeof existing === "string" && existing.trim()) continue;
      let base = path.replace(/^\//, "").replaceAll("/", "_");
      if (!base) base = "root";
      base = `${base}_${method}`;
      let candidate = base;
      let i = 2;
      while (used.has(candidate)) {
        candidate = `${base}_${i++}`;
      }
      op.operationId = candidate;
      used.add(candidate);
    }
  }
  return out;
}
function progenitorFriendly(spec) {
  const out = structuredClone(spec);
  deepWalk(out, (node) => {
    if (!node || typeof node !== "object") return;
    if (node.type === "boolean" && typeof node.format === "string") {
      delete node.format;
    }
    if (node.format === "jsonb" || node.format === "json") {
      const desc = typeof node.description === "string" ? node.description : void 0;
      for (const k of Object.keys(node)) delete node[k];
      node.allOf = [{ $ref: "#/components/schemas/PgJson" }];
      if (desc) node.description = desc;
      return;
    }
    if (!node.type) {
      if (node.properties) node.type = "object";
      else if (node.items) node.type = "array";
      else if (node.enum) node.type = "string";
    }
  });
  for (const ops of Object.values(out.paths ?? {})) {
    if (!ops || typeof ops !== "object") continue;
    for (const op of Object.values(ops)) {
      if (!op || typeof op !== "object") continue;
      const responses = op.responses;
      if (!responses || typeof responses !== "object") continue;
      for (const [status, resp] of Object.entries(responses)) {
        if (status === "200") continue;
        if (!resp || typeof resp !== "object") continue;
        const content = resp.content;
        if (!content || typeof content !== "object" || !("application/json" in content)) {
          delete responses[status];
        }
      }
    }
  }
  out.components ??= {};
  out.components.schemas ??= {};
  out.components.schemas.PgJson ??= {
    type: "object",
    additionalProperties: {},
    description: "Postgres JSON/JSONB encoded as raw JSON."
  };
  out.components.schemas.PgNumeric ??= {
    type: "number",
    description: "Postgres NUMERIC encoded as a JSON number."
  };
  deepWalk(out, (node) => {
    if (!node || typeof node !== "object") return;
    if (node.type === "number" && node.format === "numeric") {
      const desc = typeof node.description === "string" ? node.description : void 0;
      for (const k of Object.keys(node)) delete node[k];
      node.allOf = [{ $ref: "#/components/schemas/PgNumeric" }];
      if (desc) node.description = desc;
    }
  });
  return out;
}
function mergeNamed(base, extra, prefix) {
  for (const [name, obj] of Object.entries(extra)) {
    if (!(name in base)) {
      base[name] = obj;
      continue;
    }
    if (JSON.stringify(base[name]) === JSON.stringify(obj)) continue;
    base[`${prefix}${name}`] = obj;
  }
}
function mergeOpenapi3WithPrefix(base, extra, collisionPrefix) {
  const out = structuredClone(base);
  out.paths ??= {};
  for (const [path, ops] of Object.entries(extra.paths ?? {})) {
    if (!(path in out.paths)) {
      out.paths[path] = ops;
      continue;
    }
    if (typeof out.paths[path] === "object" && typeof ops === "object") {
      Object.assign(out.paths[path], ops);
    }
  }
  if (Array.isArray(out.tags) && Array.isArray(extra.tags)) {
    const existing = new Set(out.tags.map((t) => t?.name).filter(Boolean));
    for (const t of extra.tags) {
      if (t?.name && !existing.has(t.name)) out.tags.push(t);
    }
  }
  out.components ??= {};
  const outComponents = out.components;
  const extraComponents = extra.components ?? {};
  for (const kind of ["schemas", "parameters", "responses", "requestBodies"]) {
    const src = extraComponents[kind];
    if (!src || typeof src !== "object") continue;
    outComponents[kind] ??= {};
    mergeNamed(outComponents[kind], src, collisionPrefix);
  }
  return out;
}
async function convertSwagger2ToOpenapi3(swagger2) {
  if (swagger2 && typeof swagger2 === "object" && typeof swagger2.openapi === "string" && swagger2.openapi.startsWith("3.")) {
    return swagger2;
  }
  const converter = await loadSwagger2Openapi();
  const { openapi } = await converter.convertObj(swagger2, {
    patch: true,
    warnOnly: true,
    resolveInternal: true,
    targetVersion: "3.0.3"
  });
  return openapi;
}
async function buildMergedOpenapi3(params) {
  const realtor = params.realtorOpenapi3;
  let indexer = await convertSwagger2ToOpenapi3(params.upstreamSwagger2);
  indexer = keepGetOnly(indexer);
  indexer = setGatewayServers(indexer);
  indexer = progenitorFriendly(indexer);
  indexer = ensureOperationIds(indexer);
  let merged = indexer;
  if (realtor && typeof realtor === "object" && typeof realtor.openapi === "string") {
    if (realtor.openapi.startsWith("3.")) {
      merged = mergeOpenapi3WithPrefix(realtor, indexer, "indexer_");
      merged = setGatewayServers(merged);
      const indexerInfo = indexer.info;
      const mergedInfo = typeof merged.info === "object" && merged.info ? merged.info : {};
      if (typeof indexerInfo?.version === "string" && indexerInfo.version.trim()) {
        mergedInfo.version = indexerInfo.version;
      }
      merged.info = mergedInfo;
      if (realtor.openapi.startsWith("3.1")) {
        merged.openapi = "3.1.0";
      }
    }
  }
  return merged;
}

// infra/openapi-sidecar/src/cli.ts
async function readStdin() {
  return await new Promise((resolve, reject) => {
    let data = "";
    process.stdin.setEncoding("utf8");
    process.stdin.on("data", (chunk) => data += chunk);
    process.stdin.on("end", () => resolve(data));
    process.stdin.on("error", reject);
  });
}
async function main() {
  const args = process.argv.slice(2);
  const includeRealtor = args.includes("--include-realtor");
  const upstreamText = await readStdin();
  const upstreamSwagger2 = JSON.parse(upstreamText);
  let realtorOpenapi3 = null;
  if (includeRealtor) {
    const url = process.env.REALTOR_OPENAPI_URL;
    if (!url) throw new Error("REALTOR_OPENAPI_URL must be set with --include-realtor");
    const res = await fetch(url, { headers: { Accept: "application/json" } });
    if (res.ok) realtorOpenapi3 = await res.json();
  }
  const spec = await buildMergedOpenapi3({ upstreamSwagger2, realtorOpenapi3 });
  process.stdout.write(JSON.stringify(spec, null, 2) + "\n");
}
main().catch((e) => {
  console.error(String(e?.stack ?? e));
  process.exit(1);
});
