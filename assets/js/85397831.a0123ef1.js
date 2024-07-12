"use strict";(self.webpackChunk=self.webpackChunk||[]).push([[17825],{55324:(e,n,t)=>{t.d(n,{Z:()=>_});var r=t(57639),a=(t(89495),t(88917),t(47763),t(33978),t(34863)),l=t(67294),i=l.useMemo,o=l.useState,u=l.useLayoutEffect,c=l.useEffect,s={glyphMargin:!1,folding:!1,lineDecorationsWidth:10,lineNumbersMinChars:0,language:"graphql",minimap:{enabled:!1},lineNumbers:"off",automaticLayout:!0,fontSize:"16px",scrollBeyondLastLine:!1,renderIndentGuides:!1,renderLineHighlight:"none",tabSize:2};function _(e){var n,t,_,f=e.text,g=e.onDidChange,m=e.diagnostics,p=e.style,b=o(null),y=b[0],w=b[1],v=(n=o((function(){return d()})),t=n[0],_=n[1],c((function(){var e=new MutationObserver((function(e,n){for(var t,a=(0,r.Z)(e);!(t=a()).done;){var l=t.value;"attributes"===l.type&&"data-theme"===l.attributeName&&_(d())}})),n={attributes:!0};return e.observe(document.documentElement,n),function(){e.disconnect()}}),[]),t)?"vs-dark":"vs",h=i((function(){return null==y?null:a.j6.create(y,s)}),[y]);return u((function(){null!=h&&h.getValue()!==f&&h.setValue(f)}),[h,f]),u((function(){if(null!=h){var e=h.getModel();if(null!=e){var n=(null!=m?m:[]).map((function(e){return{severity:8,message:e.message,startLineNumber:e.line_start+1,startColumn:e.column_start+1,endLineNumber:e.line_end+1,endColumn:e.column_end+1}}));a.j6.setModelMarkers(e,"relay",n)}}}),[h,m]),u((function(){a.j6.setTheme(v)}),[v]),c((function(){if(null!=h&&null!=g){var e=h.onDidChangeModelContent((function(){g(h.getValue())}));return function(){e.dispose()}}}),[h,g]),l.createElement("div",{ref:w,style:p})}function d(){return"dark"===document.documentElement.dataset.theme}},91305:(e,n,t)=>{t.r(n),t.d(n,{default:()=>x});var r="\ntype User {\n  name: String\n  age: Int\n  best_friend: User\n}\n\ntype Query {\n  me: User\n}\n ".trim(),a="\nquery MyQuery {\n  me {\n    name\n    ...AgeFragment\n    best_friend {\n      ...AgeFragment\n    }\n  }\n}\n\nfragment AgeFragment on User {\n  age\n}\n ".trim(),l=[{key:"no_inline",label:"@no_inline",kind:"enum",default:!0},{key:"enable_3d_branch_arg_generation",label:"3D Branch Arg Generation",kind:"bool",default:!0},{key:"actor_change_support",label:"Actor Change Support",kind:"enum",default:!0},{key:"text_artifacts",label:"Text Artifacts",kind:"enum",default:!0},{key:"skip_printing_nulls",label:"Skip Printing Nulls",kind:"enum",default:!1},{key:"compact_query_text",label:"Compact Query Text",kind:"enum",default:!1},{key:"enable_fragment_aliases",label:"@alias",kind:"enum",default:!0},{key:"enforce_fragment_alias_where_ambiguous",label:"Enforce @alias where ambiguous",kind:"enum",default:!0},{key:"enable_catch_directive_transform",label:"@catch",kind:"enum",default:!1},{key:"disallow_required_on_non_null_fields",label:"Disallow Required on Non-Null Fields",kind:"bool",default:!1}],i={schemaText:r,documentText:a,inputWindow:"schema",outputType:"operation",featureFlags:Object.fromEntries(l.map((function(e){return[e.key,e.default]}))),language:"typescript"},o=t(57639),u=t(26961);var c=t(67294),s=c.useReducer,_=c.useMemo,d=c.useEffect,f="relayCompilerExplorerLastContent";function g(){var e=s(m,null,p),n=e[0],t=e[1];d((function(){var e=function(e){var n=new URLSearchParams;n.set("enc","1");for(var t=0,r=Object.entries(e);t<r.length;t++){var a=r[t],l=a[0],i=a[1];if("schemaText"==l||"documentText"==l)n.set(l,u.compressToEncodedURIComponent(i));else if("featureFlags"==l)for(var o=0,c=Object.entries(i);o<c.length;o++){var s=c[o],_=s[0],d=s[1];n.set(_,d)}else n.set(l,i)}return n.toString()}(n),t="#"+e;window.history.replaceState(null,null,t),localStorage.setItem(f,t)}),[n]);var r=_((function(){return{setSchemaText:function(e){return t({type:"UPDATE_SCHEMA",schemaText:e})},setDocumentText:function(e){return t({type:"UPDATE_DOCUMENT",documentText:e})},setFeatureFlag:function(e,n){return t({type:"SET_FEATURE_FLAG",flag:e,value:n})},setLanguage:function(e){return t({type:"SET_LANGUAGE",language:e})},setOutputType:function(e){return t({type:"SET_OUTPUT_TYPE",outputType:e})},setInputWindow:function(e){t({type:"UPDATE_INPUT_WINDOW",inputWindow:e})}}}),[]);return Object.assign({state:n},r)}function m(e,n){var t;switch(n.type){case"UPDATE_SCHEMA":return Object.assign({},e,{schemaText:n.schemaText});case"UPDATE_DOCUMENT":return Object.assign({},e,{documentText:n.documentText});case"SET_OUTPUT_TYPE":return Object.assign({},e,{outputType:n.outputType});case"UPDATE_INPUT_WINDOW":return Object.assign({},e,{inputWindow:n.inputWindow});case"SET_FEATURE_FLAG":var r=Object.assign({},e.featureFlags,((t={})[n.flag]=n.value,t));return Object.assign({},e,{featureFlags:r});case"SET_LANGUAGE":return Object.assign({},e,{language:n.language});default:throw new Error("Unexpected action type: "+n.type)}}function p(){var e=window.location.hash||localStorage.getItem(f);if(null!=e&&"#"===e[0]&&e.length>1){var n=e.slice(1);try{return function(e){if("1"!==e.get("enc"))return console.warn("Unexpected encoding version: "+e.get("enc")),null;for(var n=i,t=0,r=Object.keys(i);t<r.length;t++){var a=r[t],c=e.get(a);if("schemaText"==a||"documentText"==a)n[a]=u.decompressFromEncodedURIComponent(c);else if("featureFlags"==a){for(var s,_={},d=(0,o.Z)(l);!(s=d()).done;){var f=s.value.key;_[f]=Boolean(e.get(f))}n[a]=_}else n[a]=e.get(a)}return n}(new URLSearchParams(n))||i}catch(t){return console.warn("Failed to decode previous state: ",t),i}}return i}var b=t(34660),y=t(86010),w=c.useState,v=c.useEffect,h=c.useLayoutEffect,E=c.useMemo;function x(){return c.createElement(b.Z,{title:"Compiler Explorer"},c.createElement(k,{minHeight:600},c.createElement(T,null)))}function k(e){var n=e.children,t=e.minHeight,r=w(null),a=r[0],l=r[1],i=w(null),o=i[0],u=i[1];return h((function(){if(null!=a){var e=a.getBoundingClientRect().y,n=Math.max(window.innerHeight-e,t);u(n)}}),[a,t]),c.createElement("div",{style:{height:o},ref:l},null!=o&&n)}function T(){var e,n,r,a,i=g(),o=i.state,u=i.setOutputType,s=i.setInputWindow,d=i.setDocumentText,f=i.setSchemaText,m=i.setFeatureFlag,p=i.setLanguage,b=function(e){var n=e.schemaText,r=e.documentText,a=e.language,i=e.outputType,o=function(e){return _((function(){var n=Object.fromEntries(l.map((function(n){var t=n.key,r=n.kind,a=e.featureFlags[t];switch(r){case"enum":return[t,{kind:a?"enabled":"disabled"}];case"bool":return[t,a];default:throw new Error("Unexpected feature flag kind: "+r)}})));return JSON.stringify(n,null,2)}),[e.featureFlags])}(e),u=function(){var e=w(null),n=e[0],r=e[1];return v((function(){var e=!1,n=t(77056);return(0,n.default)().then((function(){e||r(n)})),function(){e=!0}}),[]),n}();return E((function(){if(null==u)return{Ok:"Loading..."};try{switch(i){case"ast":return JSON.parse(u.parse_to_ast(r));case"ir":return JSON.parse(u.parse_to_ir(n,r));case"reader":return JSON.parse(u.parse_to_reader_ast(o,n,r));case"normalization":return JSON.parse(u.parse_to_normalization_ast(o,n,r));case"operation":return JSON.parse(u.transform(o,n,r));case"types":var e=JSON.stringify({language:a});return JSON.parse(u.parse_to_types(o,e,n,r));default:throw new Error("Unknown output type "+i)}}catch(t){return{Ok:"Error: The compiler crashed: "+t.message}}}),[n,r,i,u,o,a])}(o),y=null!=(e=b.Ok)?e:"",h=null==(n=b.Err)?void 0:n.SchemaDiagnostics,x=null==(r=b.Err)?void 0:r.DocumentDiagnostics,k=null==(a=b.Err)?void 0:a.ConfigError,T=E((function(){return t(55324).Z}),[]);return c.createElement("div",{style:{height:"100%",display:"flex",flexDirection:"column",padding:20}},c.createElement("div",{style:{display:"flex",columnGap:20}},c.createElement("div",{style:{width:"50%",alignSelf:"flex-end"}},c.createElement(O,{values:[{value:"schema",label:"Schema"},{value:"document",label:"Document"},{value:"config",label:"Config"}],selectedValue:o.inputWindow,setSelectedValue:function(e){return s(e)}})),c.createElement("div",{style:{width:"50%"}},c.createElement(O,{values:[{value:"operation",label:"Operation"},{value:"ast",label:"AST"},{value:"ir",label:"IR"},{value:"normalization",label:"Normalization AST"},{value:"reader",label:"Reader AST"},{value:"types",label:"Types"}],selectedValue:o.outputType,setSelectedValue:function(e){return u(e)}}))),c.createElement("div",{style:{display:"flex",flexGrow:1,columnGap:20}},c.createElement("div",{style:{width:"50%",display:"flex",flexDirection:"column"}},function(){switch(o.inputWindow){case"schema":return c.createElement(T,{key:"schema",text:o.schemaText,onDidChange:f,style:{flexGrow:1},diagnostics:h});case"document":return c.createElement(T,{key:"document",text:o.documentText,onDidChange:d,style:{flexGrow:3},diagnostics:x});case"config":return c.createElement("div",{style:{display:"flex",padding:20,flexGrow:1,flexDirection:"column",columnGap:20}},c.createElement(S,{setFeatureFlag:m,featureFlags:o.featureFlags}),c.createElement(A,{setLanguage:p,language:o.language}))}}()),c.createElement("div",{style:{width:"50%",display:"flex"}},c.createElement("div",{style:{flexGrow:1,display:"flex",flexDirection:"column"}},c.createElement(T,{text:null!=k?k:y,style:{flexGrow:1}})))))}function S(e){var n=e.featureFlags,t=e.setFeatureFlag;return c.createElement("div",{style:{display:"grid",gridTemplateColumns:"repeat(2, 1fr)"}},l.map((function(e){var r=e.key,a=e.label;return c.createElement("label",{key:r,style:{display:"block"}},c.createElement("input",{type:"checkbox",checked:n[r],onChange:function(e){return t(r,e.target.checked)}}),a)})))}function A(e){var n=e.setLanguage,t=e.language;return c.createElement("div",null,c.createElement("label",null,"Type Generation Language:",c.createElement("select",{onChange:function(e){return n(e.target.value)},value:t},c.createElement("option",{value:"flow"},"Flow"),c.createElement("option",{value:"typescript"},"TypeScript"))))}function O(e){var n=e.values,t=e.selectedValue,r=e.setSelectedValue;return c.createElement("div",{className:"tabs-container"},c.createElement("ul",{role:"tablist","aria-orientation":"horizontal",className:"tabs"},n.map((function(e){var n=e.value,a=e.label,l=t===n;return c.createElement("li",{role:"tab",tabIndex:l?0:-1,"aria-selected":l,className:(0,y.Z)("tabs__item",{"tabs__item--active":l}),key:n,onClick:function(){return r(n)}},a)}))))}},66527:(e,n,t)=>{e.exports=t.p+"558da8701e1d339f.wasm"},77056:(e,n,t)=>{t.r(n),t.d(n,{default:()=>L,initSync:()=>C,parse_to_ast:()=>v,parse_to_ir:()=>h,parse_to_normalization_ast:()=>x,parse_to_reader_ast:()=>E,parse_to_types:()=>k,start:()=>S,transform:()=>T});var r,a=t(1017),l=t(84035),i=new Array(128).fill(void 0);function o(e){return i[e]}i.push(void 0,null,!0,!1);var u=i.length;function c(e){var n=o(e);return function(e){e<132||(i[e]=u,u=e)}(e),n}var s=0,_=null;function d(){return null!==_&&0!==_.byteLength||(_=new Uint8Array(r.memory.buffer)),_}var f="undefined"!=typeof TextEncoder?new TextEncoder("utf-8"):{encode:function(){throw Error("TextEncoder not available")}},g="function"==typeof f.encodeInto?function(e,n){return f.encodeInto(e,n)}:function(e,n){var t=f.encode(e);return n.set(t),{read:e.length,written:t.length}};function m(e,n,t){if(void 0===t){var r=f.encode(e),a=n(r.length,1)>>>0;return d().subarray(a,a+r.length).set(r),s=r.length,a}for(var l=e.length,i=n(l,1)>>>0,o=d(),u=0;u<l;u++){var c=e.charCodeAt(u);if(c>127)break;o[i+u]=c}if(u!==l){0!==u&&(e=e.slice(u)),i=t(i,l,l=u+3*e.length,1)>>>0;var _=d().subarray(i+u,i+l);i=t(i,l,u+=g(e,_).written,1)>>>0}return s=u,i}var p=null;function b(){return null!==p&&0!==p.byteLength||(p=new Int32Array(r.memory.buffer)),p}var y="undefined"!=typeof TextDecoder?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:function(){throw Error("TextDecoder not available")}};function w(e,n){return e>>>=0,y.decode(d().subarray(e,e+n))}function v(e){var n,t;try{var a=r.__wbindgen_add_to_stack_pointer(-16),l=m(e,r.__wbindgen_malloc,r.__wbindgen_realloc),i=s;r.parse_to_ast(a,l,i);var o=b()[a/4+0],u=b()[a/4+1];return n=o,t=u,w(o,u)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_free(n,t,1)}}function h(e,n){var t,a;try{var l=r.__wbindgen_add_to_stack_pointer(-16),i=m(e,r.__wbindgen_malloc,r.__wbindgen_realloc),o=s,u=m(n,r.__wbindgen_malloc,r.__wbindgen_realloc),c=s;r.parse_to_ir(l,i,o,u,c);var _=b()[l/4+0],d=b()[l/4+1];return t=_,a=d,w(_,d)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_free(t,a,1)}}function E(e,n,t){var a,l;try{var i=r.__wbindgen_add_to_stack_pointer(-16),o=m(e,r.__wbindgen_malloc,r.__wbindgen_realloc),u=s,c=m(n,r.__wbindgen_malloc,r.__wbindgen_realloc),_=s,d=m(t,r.__wbindgen_malloc,r.__wbindgen_realloc),f=s;r.parse_to_reader_ast(i,o,u,c,_,d,f);var g=b()[i/4+0],p=b()[i/4+1];return a=g,l=p,w(g,p)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_free(a,l,1)}}function x(e,n,t){var a,l;try{var i=r.__wbindgen_add_to_stack_pointer(-16),o=m(e,r.__wbindgen_malloc,r.__wbindgen_realloc),u=s,c=m(n,r.__wbindgen_malloc,r.__wbindgen_realloc),_=s,d=m(t,r.__wbindgen_malloc,r.__wbindgen_realloc),f=s;r.parse_to_normalization_ast(i,o,u,c,_,d,f);var g=b()[i/4+0],p=b()[i/4+1];return a=g,l=p,w(g,p)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_free(a,l,1)}}function k(e,n,t,a){var l,i;try{var o=r.__wbindgen_add_to_stack_pointer(-16),u=m(e,r.__wbindgen_malloc,r.__wbindgen_realloc),c=s,_=m(n,r.__wbindgen_malloc,r.__wbindgen_realloc),d=s,f=m(t,r.__wbindgen_malloc,r.__wbindgen_realloc),g=s,p=m(a,r.__wbindgen_malloc,r.__wbindgen_realloc),y=s;r.parse_to_types(o,u,c,_,d,f,g,p,y);var v=b()[o/4+0],h=b()[o/4+1];return l=v,i=h,w(v,h)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_free(l,i,1)}}function T(e,n,t){var a,l;try{var i=r.__wbindgen_add_to_stack_pointer(-16),o=m(e,r.__wbindgen_malloc,r.__wbindgen_realloc),u=s,c=m(n,r.__wbindgen_malloc,r.__wbindgen_realloc),_=s,d=m(t,r.__wbindgen_malloc,r.__wbindgen_realloc),f=s;r.transform(i,o,u,c,_,d,f);var g=b()[i/4+0],p=b()[i/4+1];return a=g,l=p,w(g,p)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_free(a,l,1)}}function S(){r.start()}function A(e,n){return O.apply(this,arguments)}function O(){return(O=(0,l.Z)((0,a.Z)().mark((function e(n,t){var r,l;return(0,a.Z)().wrap((function(e){for(;;)switch(e.prev=e.next){case 0:if(!("function"==typeof Response&&n instanceof Response)){e.next=23;break}if("function"!=typeof WebAssembly.instantiateStreaming){e.next=15;break}return e.prev=2,e.next=5,WebAssembly.instantiateStreaming(n,t);case 5:case 20:return e.abrupt("return",e.sent);case 8:if(e.prev=8,e.t0=e.catch(2),"application/wasm"==n.headers.get("Content-Type")){e.next=14;break}console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e.t0),e.next=15;break;case 14:throw e.t0;case 15:return e.next=17,n.arrayBuffer();case 17:return r=e.sent,e.next=20,WebAssembly.instantiate(r,t);case 23:return e.next=25,WebAssembly.instantiate(n,t);case 25:if(!((l=e.sent)instanceof WebAssembly.Instance)){e.next=30;break}return e.abrupt("return",{instance:l,module:n});case 30:return e.abrupt("return",l);case 31:case"end":return e.stop()}}),e,null,[[2,8]])})))).apply(this,arguments)}function U(){var e={wbg:{}};return e.wbg.__wbg_new_abda76e883ba8a5f=function(){return function(e){u===i.length&&i.push(i.length+1);var n=u;return u=i[n],i[n]=e,n}(new Error)},e.wbg.__wbg_stack_658279fe44541cf6=function(e,n){var t=m(o(n).stack,r.__wbindgen_malloc,r.__wbindgen_realloc),a=s;b()[e/4+1]=a,b()[e/4+0]=t},e.wbg.__wbg_error_f851667af71bcfc6=function(e,n){var t,a;try{t=e,a=n,console.error(w(e,n))}finally{r.__wbindgen_free(t,a,1)}},e.wbg.__wbindgen_object_drop_ref=function(e){c(e)},e}function D(e,n){return r=e.exports,F.__wbindgen_wasm_module=n,p=null,_=null,r.__wbindgen_start(),r}function C(e){if(void 0!==r)return r;var n=U();return e instanceof WebAssembly.Module||(e=new WebAssembly.Module(e)),D(new WebAssembly.Instance(e,n),e)}function F(e){return N.apply(this,arguments)}function N(){return(N=(0,l.Z)((0,a.Z)().mark((function e(n){var l,i,o,u;return(0,a.Z)().wrap((function(e){for(;;)switch(e.prev=e.next){case 0:if(void 0===r){e.next=2;break}return e.abrupt("return",r);case 2:return void 0===n&&(n=new URL(t(66527),t.b)),l=U(),("string"==typeof n||"function"==typeof Request&&n instanceof Request||"function"==typeof URL&&n instanceof URL)&&(n=fetch(n)),e.t0=A,e.next=9,n;case 9:return e.t1=e.sent,e.t2=l,e.next=13,(0,e.t0)(e.t1,e.t2);case 13:return i=e.sent,o=i.instance,u=i.module,e.abrupt("return",D(o,u));case 17:case"end":return e.stop()}}),e)})))).apply(this,arguments)}"undefined"!=typeof TextDecoder&&y.decode();const L=F}}]);