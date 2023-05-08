/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".bootstrap.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/wp_test_bg.wasm": function() {
/******/ 			return {
/******/ 				"./wp_test_bg.js": {
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_Number_4d891a463a5b8707": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_Number_4d891a463a5b8707"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_log_33328a2f2101b1c3": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_log_33328a2f2101b1c3"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_new_abda76e883ba8a5f": function() {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_new_abda76e883ba8a5f"]();
/******/ 					},
/******/ 					"__wbg_stack_658279fe44541cf6": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_stack_658279fe44541cf6"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_f851667af71bcfc6": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_error_f851667af71bcfc6"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_e266f02eee43b570": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_instanceof_Window_e266f02eee43b570"](p0i32);
/******/ 					},
/******/ 					"__wbg_document_950215a728589a2d": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_document_950215a728589a2d"](p0i32);
/******/ 					},
/******/ 					"__wbg_body_be46234bb33edd63": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_body_be46234bb33edd63"](p0i32);
/******/ 					},
/******/ 					"__wbg_createElement_e2a0e21263eb5416": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_createElement_e2a0e21263eb5416"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_querySelector_32b9d7ebb2df951d": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_querySelector_32b9d7ebb2df951d"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_querySelectorAll_608b5716e2a3baf0": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_querySelectorAll_608b5716e2a3baf0"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_setcssText_4b5b31f23dd83c99": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_setcssText_4b5b31f23dd83c99"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_addEventListener_615d4590d38da1c9": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_addEventListener_615d4590d38da1c9"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_length_c54fcfc679a5bfbd": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_length_c54fcfc679a5bfbd"](p0i32);
/******/ 					},
/******/ 					"__wbg_item_b070543ecdd3aeeb": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_item_b070543ecdd3aeeb"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlParagraphElement_5288568d348153a5": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_instanceof_HtmlParagraphElement_5288568d348153a5"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlElement_9e442d53bb553421": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_instanceof_HtmlElement_9e442d53bb553421"](p0i32);
/******/ 					},
/******/ 					"__wbg_style_2141664e428fef46": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_style_2141664e428fef46"](p0i32);
/******/ 					},
/******/ 					"__wbg_focus_6497e1b44dabfb24": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_focus_6497e1b44dabfb24"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlInputElement_5c9d54338207f061": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_instanceof_HtmlInputElement_5c9d54338207f061"](p0i32);
/******/ 					},
/******/ 					"__wbg_setdisabled_3fceae4265ead699": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_setdisabled_3fceae4265ead699"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_value_1f2c9e357d18d3ea": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_value_1f2c9e357d18d3ea"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setvalue_a706abe70dab1b65": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_setvalue_a706abe70dab1b65"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_parentNode_e81e6d5dc2fc35b0": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_parentNode_e81e6d5dc2fc35b0"](p0i32);
/******/ 					},
/******/ 					"__wbg_textContent_dff59ad5e030bb86": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_textContent_dff59ad5e030bb86"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_settextContent_19dc6a6146112f16": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_settextContent_19dc6a6146112f16"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_appendChild_b8199dc1655c852d": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_appendChild_b8199dc1655c852d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_removeChild_794db72cbb6f21d3": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_removeChild_794db72cbb6f21d3"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlButtonElement_7046caffb25a7bfb": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_instanceof_HtmlButtonElement_7046caffb25a7bfb"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_2b8b6bd7753c76ba": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_newnoargs_2b8b6bd7753c76ba"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_95d1ea488d03e4e8": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_call_95d1ea488d03e4e8"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_self_e7c1f827057f6584": function() {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_self_e7c1f827057f6584"]();
/******/ 					},
/******/ 					"__wbg_window_a09ec664e14b1b81": function() {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_window_a09ec664e14b1b81"]();
/******/ 					},
/******/ 					"__wbg_globalThis_87cbb8506fecf3a9": function() {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_globalThis_87cbb8506fecf3a9"]();
/******/ 					},
/******/ 					"__wbg_global_c85a9259e621f3db": function() {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_global_c85a9259e621f3db"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_floor_edc79eb6ad54781b": function(p0f64) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_floor_edc79eb6ad54781b"](p0f64);
/******/ 					},
/******/ 					"__wbg_random_afb3265527cf67c8": function() {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbg_random_afb3265527cf67c8"]();
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper83": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wp_test_bg.js"].exports["__wbindgen_closure_wrapper83"](p0i32,p1i32,p2i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/wp_test_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/wp_test_bg.wasm":"243846f48e25eb18636a"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// A dependency graph that contains any wasm must all be imported\r\n// asynchronously. This `bootstrap.js` file does the single async import, so\r\n// that no one else needs to worry about it again.\r\n__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./index.js */ \"./index.js\"))\r\n  .catch(e => console.error(\"Error importing `index.js`:\", e));\r\n\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });