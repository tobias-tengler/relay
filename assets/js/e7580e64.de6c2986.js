(self.webpackChunk=self.webpackChunk||[]).push([[44559,27950,81046,43218,98275],{3905:(e,t,n)=>{"use strict";n.r(t),n.d(t,{MDXContext:()=>c,MDXProvider:()=>u,mdx:()=>v,useMDXComponents:()=>p,withMDXComponents:()=>d});var r=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(){return i=Object.assign||function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var r in n)Object.prototype.hasOwnProperty.call(n,r)&&(e[r]=n[r])}return e},i.apply(this,arguments)}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var c=r.createContext({}),d=function(e){return function(t){var n=p(t.components);return r.createElement(e,i({},t,{components:n}))}},p=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},u=function(e){var t=p(e.components);return r.createElement(c.Provider,{value:t},e.children)},m={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},f=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,i=e.originalType,o=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),d=p(n),u=a,f=d["".concat(o,".").concat(u)]||d[u]||m[u]||i;return n?r.createElement(f,s(s({ref:t},c),{},{components:n})):r.createElement(f,s({ref:t},c))}));function v(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=n.length,o=new Array(i);o[0]=f;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s.mdxType="string"==typeof e?e:a,o[1]=s;for(var c=2;c<i;c++)o[c]=n[c];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}f.displayName="MDXCreateElement"},36742:(e,t,n)=>{"use strict";n.r(t),n.d(t,{default:()=>m});var r=n(79973),a=n(67294),i=n(73727),o=n(52263),s=n(13919),l=n(10412),c=(0,a.createContext)({collectLink:function(){}}),d=n(44996),p=n(18780),u=["isNavLink","to","href","activeClassName","isActive","data-noBrokenLinkCheck","autoAddBaseUrl"];const m=function(e){var t,n,m=e.isNavLink,f=e.to,v=e.href,h=e.activeClassName,b=e.isActive,y=e["data-noBrokenLinkCheck"],x=e.autoAddBaseUrl,g=void 0===x||x,N=(0,r.Z)(e,u),C=(0,o.default)().siteConfig,w=C.trailingSlash,S=C.baseUrl,k=(0,d.useBaseUrlUtils)().withBaseUrl,U=(0,a.useContext)(c),D=f||v,T=(0,s.Z)(D),O=null==D?void 0:D.replace("pathname://",""),A=void 0!==O?(n=O,g&&function(e){return e.startsWith("/")}(n)?k(n):n):void 0;A&&T&&(A=(0,p.applyTrailingSlash)(A,{trailingSlash:w,baseUrl:S}));var L=(0,a.useRef)(!1),q=m?i.OL:i.rU,E=l.default.canUseIntersectionObserver,P=(0,a.useRef)();(0,a.useEffect)((function(){return!E&&T&&null!=A&&window.docusaurus.prefetch(A),function(){E&&P.current&&P.current.disconnect()}}),[P,A,E,T]);var M=null!==(t=null==A?void 0:A.startsWith("#"))&&void 0!==t&&t,j=!A||!T||M;return A&&T&&!M&&!y&&U.collectLink(A),j?a.createElement("a",Object.assign({href:A},D&&!T&&{target:"_blank",rel:"noopener noreferrer"},N)):a.createElement(q,Object.assign({},N,{onMouseEnter:function(){L.current||null==A||(window.docusaurus.preload(A),L.current=!0)},innerRef:function(e){var t,n;E&&e&&T&&(t=e,n=function(){null!=A&&window.docusaurus.prefetch(A)},P.current=new window.IntersectionObserver((function(e){e.forEach((function(e){t===e.target&&(e.isIntersecting||e.intersectionRatio>0)&&(P.current.unobserve(t),P.current.disconnect(),n())}))})),P.current.observe(t))},to:A||""},m&&{isActive:b,activeClassName:h}))}},13919:(e,t,n)=>{"use strict";function r(e){return!0===/^(\w*:|\/\/)/.test(e)}function a(e){return void 0!==e&&!r(e)}n.d(t,{b:()=>r,Z:()=>a})},44996:(e,t,n)=>{"use strict";n.r(t),n.d(t,{useBaseUrlUtils:()=>i,default:()=>o});var r=n(52263),a=n(13919);function i(){var e=(0,r.default)().siteConfig,t=(e=void 0===e?{}:e).baseUrl,n=void 0===t?"/":t,i=e.url;return{withBaseUrl:function(e,t){return function(e,t,n,r){var i=void 0===r?{}:r,o=i.forcePrependBaseUrl,s=void 0!==o&&o,l=i.absolute,c=void 0!==l&&l;if(!n)return n;if(n.startsWith("#"))return n;if((0,a.b)(n))return n;if(s)return t+n;var d=n.startsWith(t)?n:t+n.replace(/^\//,"");return c?e+d:d}(i,n,e,t)}}}function o(e,t){return void 0===t&&(t={}),(0,i().withBaseUrl)(e,t)}},8802:(e,t)=>{"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.default=function(e,t){var n=t.trailingSlash,r=t.baseUrl;if(e.startsWith("#"))return e;if(void 0===n)return e;var a,i=e.split(/[#?]/)[0],o="/"===i||i===r?i:(a=i,n?function(e){return e.endsWith("/")?e:e+"/"}(a):function(e){return e.endsWith("/")?e.slice(0,-1):e}(a));return e.replace(i,o)}},18780:function(e,t,n){"use strict";var r=this&&this.__importDefault||function(e){return e&&e.__esModule?e:{default:e}};Object.defineProperty(t,"__esModule",{value:!0}),t.uniq=t.applyTrailingSlash=void 0;var a=n(8802);Object.defineProperty(t,"applyTrailingSlash",{enumerable:!0,get:function(){return r(a).default}});var i=n(29964);Object.defineProperty(t,"uniq",{enumerable:!0,get:function(){return r(i).default}})},29964:(e,t)=>{"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.default=function(e){return Array.from(new Set(e))}},68629:(e,t,n)=>{"use strict";n.d(t,{Z:()=>m});var r=n(36742),a=n(44256),i=n(67294);function o(){var e=window.encodeURI(JSON.stringify({title:"Feedback about "+window.location.pathname,description:"**!!! Required !!!**\n\nPlease modify the task description to let us know how the docs can be improved.\n\n**Please do not ask support questions via this form! Instead, ask in fburl.com/relay_support**",tag_ids:{add:[0xac96423e5b680,0x64079768ac750]}}));window.open("https://www.internalfb.com/tasks/?n="+e)}function s(e){var t=e.children;return i.createElement("div",{className:"docsRating",id:"docsRating"},i.createElement("hr",null),t)}var l=function(){var e=i.useState(!1),t=e[0],n=e[1],r=function(e){n(!0),function(e){window.ga&&window.ga("send",{hitType:"event",eventCategory:"button",eventAction:"feedback",eventValue:e})}(e)};return t?"Thank you for letting us know!":i.createElement(i.Fragment,null,"Is this page useful?",i.createElement("svg",{className:"i_thumbsup",alt:"Like",id:"docsRating-like",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 81.13 89.76",onClick:function(){return r(1)}},i.createElement("path",{d:"M22.9 6a18.57 18.57 0 002.67 8.4 25.72 25.72 0 008.65 7.66c3.86 2 8.67 7.13 13.51 11 3.86 3.11 8.57 7.11 11.54 8.45s13.59.26 14.64 1.17c1.88 1.63 1.55 9-.11 15.25-1.61 5.86-5.96 10.55-6.48 16.86-.4 4.83-2.7 4.88-10.93 4.88h-1.35c-3.82 0-8.24 2.93-12.92 3.62a68 68 0 01-9.73.5c-3.57 0-7.86-.08-13.25-.08-3.56 0-4.71-1.83-4.71-4.48h8.42a3.51 3.51 0 000-7H12.28a2.89 2.89 0 01-2.88-2.88 1.91 1.91 0 01.77-1.78h16.46a3.51 3.51 0 000-7H12.29c-3.21 0-4.84-1.83-4.84-4a6.41 6.41 0 011.17-3.78h19.06a3.5 3.5 0 100-7H9.75A3.51 3.51 0 016 42.27a3.45 3.45 0 013.75-3.48h13.11c5.61 0 7.71-3 5.71-5.52-4.43-4.74-10.84-12.62-11-18.71-.15-6.51 2.6-7.83 5.36-8.56m0-6a6.18 6.18 0 00-1.53.2c-6.69 1.77-10 6.65-9.82 14.5.08 5.09 2.99 11.18 8.52 18.09H9.74a9.52 9.52 0 00-6.23 16.9 12.52 12.52 0 00-2.07 6.84 9.64 9.64 0 003.65 7.7 7.85 7.85 0 00-1.7 5.13 8.9 8.9 0 005.3 8.13 6 6 0 00-.26 1.76c0 6.37 4.2 10.48 10.71 10.48h13.25a73.75 73.75 0 0010.6-.56 35.89 35.89 0 007.58-2.18 17.83 17.83 0 014.48-1.34h1.35c4.69 0 7.79 0 10.5-1 3.85-1.44 6-4.59 6.41-9.38.2-2.46 1.42-4.85 2.84-7.62a41.3 41.3 0 003.42-8.13 48 48 0 001.59-10.79c.1-5.13-1-8.48-3.35-10.55-2.16-1.87-4.64-1.87-9.6-1.88a46.86 46.86 0 01-6.64-.29c-1.92-.94-5.72-4-8.51-6.3l-1.58-1.28c-1.6-1.3-3.27-2.79-4.87-4.23-3.33-3-6.47-5.79-9.61-7.45a20.2 20.2 0 01-6.43-5.53 12.44 12.44 0 01-1.72-5.36 6 6 0 00-6-5.86z"})),i.createElement("svg",{className:"i_thumbsdown",alt:"Dislike",id:"docsRating-dislike",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 81.13 89.76",onClick:function(){return r(0)}},i.createElement("path",{d:"M22.9 6a18.57 18.57 0 002.67 8.4 25.72 25.72 0 008.65 7.66c3.86 2 8.67 7.13 13.51 11 3.86 3.11 8.57 7.11 11.54 8.45s13.59.26 14.64 1.17c1.88 1.63 1.55 9-.11 15.25-1.61 5.86-5.96 10.55-6.48 16.86-.4 4.83-2.7 4.88-10.93 4.88h-1.35c-3.82 0-8.24 2.93-12.92 3.62a68 68 0 01-9.73.5c-3.57 0-7.86-.08-13.25-.08-3.56 0-4.71-1.83-4.71-4.48h8.42a3.51 3.51 0 000-7H12.28a2.89 2.89 0 01-2.88-2.88 1.91 1.91 0 01.77-1.78h16.46a3.51 3.51 0 000-7H12.29c-3.21 0-4.84-1.83-4.84-4a6.41 6.41 0 011.17-3.78h19.06a3.5 3.5 0 100-7H9.75A3.51 3.51 0 016 42.27a3.45 3.45 0 013.75-3.48h13.11c5.61 0 7.71-3 5.71-5.52-4.43-4.74-10.84-12.62-11-18.71-.15-6.51 2.6-7.83 5.36-8.56m0-6a6.18 6.18 0 00-1.53.2c-6.69 1.77-10 6.65-9.82 14.5.08 5.09 2.99 11.18 8.52 18.09H9.74a9.52 9.52 0 00-6.23 16.9 12.52 12.52 0 00-2.07 6.84 9.64 9.64 0 003.65 7.7 7.85 7.85 0 00-1.7 5.13 8.9 8.9 0 005.3 8.13 6 6 0 00-.26 1.76c0 6.37 4.2 10.48 10.71 10.48h13.25a73.75 73.75 0 0010.6-.56 35.89 35.89 0 007.58-2.18 17.83 17.83 0 014.48-1.34h1.35c4.69 0 7.79 0 10.5-1 3.85-1.44 6-4.59 6.41-9.38.2-2.46 1.42-4.85 2.84-7.62a41.3 41.3 0 003.42-8.13 48 48 0 001.59-10.79c.1-5.13-1-8.48-3.35-10.55-2.16-1.87-4.64-1.87-9.6-1.88a46.86 46.86 0 01-6.64-.29c-1.92-.94-5.72-4-8.51-6.3l-1.58-1.28c-1.6-1.3-3.27-2.79-4.87-4.23-3.33-3-6.47-5.79-9.61-7.45a20.2 20.2 0 01-6.43-5.53 12.44 12.44 0 01-1.72-5.36 6 6 0 00-6-5.86z"})))},c=function(){return i.createElement("p",null,"Let us know how these docs can be improved by",i.createElement("a",{className:"button",role:"button",tabIndex:0,onClick:o},"Filing a task"))},d=function(){return i.createElement("p",null,"Help us make the site even better by"," ",i.createElement(r.default,{to:"https://www.surveymonkey.com/r/FYC9TCJ"},"answering a few quick questions"),".")},p=function(){return i.createElement(s,null,i.createElement(c,null),i.createElement(l,null),i.createElement(d,null))},u=function(){return i.createElement(s,null,i.createElement(l,null),i.createElement(d,null))};const m=function(){return(0,a.fbContent)({internal:i.createElement(p,null),external:i.createElement(u,null)})}},2617:(e,t,n)=>{"use strict";n.r(t),n.d(t,{frontMatter:()=>d,contentTitle:()=>p,metadata:()=>u,toc:()=>m,default:()=>v});var r=n(74034),a=n(79973),i=(n(67294),n(3905)),o=n(68629),s=(n(44256),n(15461)),l=n(31711),c=["components"],d={id:"request-subscription",title:"requestSubscription",slug:"/api-reference/request-subscription/"},p=void 0,u={unversionedId:"api-reference/relay-runtime/request-subscription",id:"version-v11.0.0/api-reference/relay-runtime/request-subscription",isDocsHomePage:!1,title:"requestSubscription",description:"requestSubscription",source:"@site/versioned_docs/version-v11.0.0/api-reference/relay-runtime/request-subscription.md",sourceDirName:"api-reference/relay-runtime",slug:"/api-reference/request-subscription/",permalink:"/docs/v11.0.0/api-reference/request-subscription/",editUrl:"https://github.com/facebook/relay/tree/main/website/versioned_docs/version-v11.0.0/api-reference/relay-runtime/request-subscription.md",tags:[],version:"v11.0.0",lastUpdatedBy:"Andrey Lunyov",lastUpdatedAt:1650564917,formattedLastUpdatedAt:"4/21/2022",frontMatter:{id:"request-subscription",title:"requestSubscription",slug:"/api-reference/request-subscription/"},sidebar:"version-v11.0.0/docs",previous:{title:"commitMutation",permalink:"/docs/v11.0.0/api-reference/commit-mutation/"},next:{title:"GraphQL Directives",permalink:"/docs/v11.0.0/api-reference/graphql-and-directives/"}},m=[{value:"<code>requestSubscription</code>",id:"requestsubscription",children:[{value:"Arguments",id:"arguments",children:[],level:3},{value:"Flow Type Parameters",id:"flow-type-parameters",children:[],level:3},{value:"Return Type",id:"return-type",children:[],level:3},{value:"Behavior",id:"behavior",children:[],level:3}],level:2}],f={toc:m};function v(e){var t=e.components,n=(0,a.Z)(e,c);return(0,i.mdx)("wrapper",(0,r.Z)({},f,n,{components:t,mdxType:"MDXLayout"}),(0,i.mdx)("h2",{id:"requestsubscription"},(0,i.mdx)("inlineCode",{parentName:"h2"},"requestSubscription")),(0,i.mdx)("p",null,"Imperative API for establishing a GraphQL Subscription.\nSee also the ",(0,i.mdx)("a",{parentName:"p",href:"../use-subscription/"},(0,i.mdx)("inlineCode",{parentName:"a"},"useSubscription"))," API and the ",(0,i.mdx)("a",{parentName:"p",href:"../../guided-tour/updating-data/"},"Guide to Updating Data"),"."),(0,i.mdx)("pre",null,(0,i.mdx)("code",{parentName:"pre",className:"language-js"},"import {graphql, requestSubscription} from 'react-relay';\n\nconst subscription = graphql`\n  subscription UserDataSubscription($input: InputData!) {\n    # ...\n  }\n`;\n\nfunction createSubscription(environment: IEnvironment): Disposable {\n  return requestSubscription(environment, {\n    subscription,\n    variables: {input: {userId: '4'}},\n  });\n}\n")),(0,i.mdx)("h3",{id:"arguments"},"Arguments"),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"environment"),": A Relay Environment"),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"config"),": ",(0,i.mdx)("inlineCode",{parentName:"li"},"GraphQLSubscriptionConfig"))),(0,i.mdx)(s.default,{mdxType:"GraphQLSubscriptionConfig"}),(0,i.mdx)("h3",{id:"flow-type-parameters"},"Flow Type Parameters"),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"TSubscriptionPayload"),": The type of the payloads vended by the subscription. You should pass the flow type imported from the auto-generated ",(0,i.mdx)("inlineCode",{parentName:"li"},".graphql")," file corresponding to the subscription, e.g. use ",(0,i.mdx)("inlineCode",{parentName:"li"},"UserDataSubscription")," as the type parameter, from ",(0,i.mdx)("inlineCode",{parentName:"li"},"import type {UserDataSubscription} from './__generated__/UserDataSubscription.graphql'"),";")),(0,i.mdx)("h3",{id:"return-type"},"Return Type"),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},"A ",(0,i.mdx)("a",{parentName:"li",href:"#interface-disposable"},(0,i.mdx)("inlineCode",{parentName:"a"},"Disposable"))," that clears the subscription.")),(0,i.mdx)(l.default,{mdxType:"Disposable"}),(0,i.mdx)("h3",{id:"behavior"},"Behavior"),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},"Imperatively establish a subscription."),(0,i.mdx)("li",{parentName:"ul"},"See the ",(0,i.mdx)("a",{parentName:"li",href:"../../guided-tour/updating-data/graphql-subscriptions/"},"GraphQL Subscriptions Guide")," for a more detailed explanation of how to work with subscriptions.")),(0,i.mdx)(o.Z,{mdxType:"DocsRating"}))}v.isMDXComponent=!0},13394:(e,t,n)=>{"use strict";n.r(t),n.d(t,{frontMatter:()=>s,contentTitle:()=>l,metadata:()=>c,toc:()=>d,default:()=>u});var r=n(74034),a=n(79973),i=(n(67294),n(3905)),o=["components"],s={},l=void 0,c={unversionedId:"api-reference/types/CacheConfig",id:"version-v11.0.0/api-reference/types/CacheConfig",isDocsHomePage:!1,title:"CacheConfig",description:"Type CacheConfig",source:"@site/versioned_docs/version-v11.0.0/api-reference/types/CacheConfig.md",sourceDirName:"api-reference/types",slug:"/api-reference/types/CacheConfig",permalink:"/docs/v11.0.0/api-reference/types/CacheConfig",editUrl:"https://github.com/facebook/relay/tree/main/website/versioned_docs/version-v11.0.0/api-reference/types/CacheConfig.md",tags:[],version:"v11.0.0",lastUpdatedBy:"Andrey Lunyov",lastUpdatedAt:1650564917,formattedLastUpdatedAt:"4/21/2022",frontMatter:{}},d=[{value:"Type <code>CacheConfig</code>",id:"type-cacheconfig",children:[],level:4}],p={toc:d};function u(e){var t=e.components,n=(0,a.Z)(e,o);return(0,i.mdx)("wrapper",(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,i.mdx)("h4",{id:"type-cacheconfig"},"Type ",(0,i.mdx)("inlineCode",{parentName:"h4"},"CacheConfig")),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},"An object with the following fields:",(0,i.mdx)("ul",{parentName:"li"},(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"force"),": ",(0,i.mdx)("em",{parentName:"li"},(0,i.mdx)("em",{parentName:"em"},"[Optional]"))," A boolean. If true, causes a query to be issued unconditionally, regardless of the state of any configured response cache."),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"poll"),": ",(0,i.mdx)("em",{parentName:"li"},(0,i.mdx)("em",{parentName:"em"},"[Optional]"))," A number. Causes a query to live-update by polling at the specified interval, in milliseconds. (This value will be passed to ",(0,i.mdx)("inlineCode",{parentName:"li"},"setTimeout"),")."),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"liveConfigId"),": ",(0,i.mdx)("em",{parentName:"li"},(0,i.mdx)("em",{parentName:"em"},"[Optional]"))," A string. Causes a query to live-update by calling GraphQLLiveQuery; it represents a configuration of gateway when doing live query."),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"metadata"),": ",(0,i.mdx)("em",{parentName:"li"},(0,i.mdx)("em",{parentName:"em"},"[Optional]"))," An object. User-supplied metadata."),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"transactionId"),": ",(0,i.mdx)("em",{parentName:"li"},(0,i.mdx)("em",{parentName:"em"},"[Optional]"))," A string. A user-supplied value, intended for use as a unique id for a given instance of executing an operation.")))))}u.isMDXComponent=!0},31711:(e,t,n)=>{"use strict";n.r(t),n.d(t,{frontMatter:()=>s,contentTitle:()=>l,metadata:()=>c,toc:()=>d,default:()=>u});var r=n(74034),a=n(79973),i=(n(67294),n(3905)),o=["components"],s={},l=void 0,c={unversionedId:"api-reference/types/Disposable",id:"version-v11.0.0/api-reference/types/Disposable",isDocsHomePage:!1,title:"Disposable",description:"Interface Disposable",source:"@site/versioned_docs/version-v11.0.0/api-reference/types/Disposable.md",sourceDirName:"api-reference/types",slug:"/api-reference/types/Disposable",permalink:"/docs/v11.0.0/api-reference/types/Disposable",editUrl:"https://github.com/facebook/relay/tree/main/website/versioned_docs/version-v11.0.0/api-reference/types/Disposable.md",tags:[],version:"v11.0.0",lastUpdatedBy:"Andrey Lunyov",lastUpdatedAt:1650564917,formattedLastUpdatedAt:"4/21/2022",frontMatter:{}},d=[{value:"Interface <code>Disposable</code>",id:"interface-disposable",children:[],level:4}],p={toc:d};function u(e){var t=e.components,n=(0,a.Z)(e,o);return(0,i.mdx)("wrapper",(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,i.mdx)("h4",{id:"interface-disposable"},"Interface ",(0,i.mdx)("inlineCode",{parentName:"h4"},"Disposable")),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},"An object with the following key:",(0,i.mdx)("ul",{parentName:"li"},(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"dispose"),": ",(0,i.mdx)("inlineCode",{parentName:"li"},"() => void"),". Disposes of the resource.")))))}u.isMDXComponent=!0},15461:(e,t,n)=>{"use strict";n.r(t),n.d(t,{frontMatter:()=>c,contentTitle:()=>d,metadata:()=>p,toc:()=>u,default:()=>f});var r=n(74034),a=n(79973),i=(n(67294),n(3905)),o=n(77193),s=n(13394),l=["components"],c={},d=void 0,p={unversionedId:"api-reference/types/GraphQLSubscriptionConfig",id:"version-v11.0.0/api-reference/types/GraphQLSubscriptionConfig",isDocsHomePage:!1,title:"GraphQLSubscriptionConfig",description:"Type GraphQLSubscriptionConfig",source:"@site/versioned_docs/version-v11.0.0/api-reference/types/GraphQLSubscriptionConfig.md",sourceDirName:"api-reference/types",slug:"/api-reference/types/GraphQLSubscriptionConfig",permalink:"/docs/v11.0.0/api-reference/types/GraphQLSubscriptionConfig",editUrl:"https://github.com/facebook/relay/tree/main/website/versioned_docs/version-v11.0.0/api-reference/types/GraphQLSubscriptionConfig.md",tags:[],version:"v11.0.0",lastUpdatedBy:"Andrey Lunyov",lastUpdatedAt:1650564917,formattedLastUpdatedAt:"4/21/2022",frontMatter:{}},u=[{value:"Type <code>GraphQLSubscriptionConfig&lt;TSubscriptionPayload&gt;</code>",id:"type-graphqlsubscriptionconfigtsubscriptionpayload",children:[],level:4}],m={toc:u};function f(e){var t=e.components,n=(0,a.Z)(e,l);return(0,i.mdx)("wrapper",(0,r.Z)({},m,n,{components:t,mdxType:"MDXLayout"}),(0,i.mdx)("h4",{id:"type-graphqlsubscriptionconfigtsubscriptionpayload"},"Type ",(0,i.mdx)("inlineCode",{parentName:"h4"},"GraphQLSubscriptionConfig<TSubscriptionPayload>")),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},"An object with the following fields:",(0,i.mdx)("ul",{parentName:"li"},(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"cacheConfig"),": ",(0,i.mdx)("em",{parentName:"li"},(0,i.mdx)("em",{parentName:"em"},"[Optional]"))," ",(0,i.mdx)("a",{parentName:"li",href:"#type-cacheconfig"},(0,i.mdx)("inlineCode",{parentName:"a"},"CacheConfig"))),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"subscription"),": ",(0,i.mdx)("inlineCode",{parentName:"li"},"GraphQLTaggedNode"),". A GraphQL subscription specified using a ",(0,i.mdx)("inlineCode",{parentName:"li"},"graphql")," template literal"),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"variables"),": The variables to pass to the subscription"),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"onCompleted"),": ",(0,i.mdx)("em",{parentName:"li"},(0,i.mdx)("em",{parentName:"em"},"[Optional]"))," ",(0,i.mdx)("inlineCode",{parentName:"li"},"() => void"),". An optional callback that is executed when the subscription is established"),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"onError"),": ",(0,i.mdx)("em",{parentName:"li"},(0,i.mdx)("em",{parentName:"em"},"[Optional]"))," ",(0,i.mdx)("inlineCode",{parentName:"li"},"(Error) => {}"),". An optional callback that is executed when an error occurs"),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"onNext"),": ",(0,i.mdx)("em",{parentName:"li"},(0,i.mdx)("em",{parentName:"em"},"[Optional]"))," ",(0,i.mdx)("inlineCode",{parentName:"li"},"(TSubscriptionPayload) => {}"),". An optional callback that is executed when new data is received"),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"updater"),": ",(0,i.mdx)("em",{parentName:"li"},(0,i.mdx)("em",{parentName:"em"},"[Optional]"))," ",(0,i.mdx)("a",{parentName:"li",href:"#type-selectorstoreupdater"},(0,i.mdx)("inlineCode",{parentName:"a"},"SelectorStoreUpdater")),".")))),(0,i.mdx)(s.default,{mdxType:"CacheConfig"}),(0,i.mdx)(o.default,{mdxType:"SelectorStoreUpdater"}))}f.isMDXComponent=!0},77193:(e,t,n)=>{"use strict";n.r(t),n.d(t,{frontMatter:()=>l,contentTitle:()=>c,metadata:()=>d,toc:()=>p,default:()=>m});var r=n(74034),a=n(79973),i=(n(67294),n(3905)),o=n(44996),s=["components"],l={},c=void 0,d={unversionedId:"api-reference/types/SelectorStoreUpdater",id:"version-v11.0.0/api-reference/types/SelectorStoreUpdater",isDocsHomePage:!1,title:"SelectorStoreUpdater",description:"Type SelectorStoreUpdater",source:"@site/versioned_docs/version-v11.0.0/api-reference/types/SelectorStoreUpdater.md",sourceDirName:"api-reference/types",slug:"/api-reference/types/SelectorStoreUpdater",permalink:"/docs/v11.0.0/api-reference/types/SelectorStoreUpdater",editUrl:"https://github.com/facebook/relay/tree/main/website/versioned_docs/version-v11.0.0/api-reference/types/SelectorStoreUpdater.md",tags:[],version:"v11.0.0",lastUpdatedBy:"Andrey Lunyov",lastUpdatedAt:1650564917,formattedLastUpdatedAt:"4/21/2022",frontMatter:{}},p=[{value:"Type <code>SelectorStoreUpdater</code>",id:"type-selectorstoreupdater",children:[],level:4}],u={toc:p};function m(e){var t=e.components,n=(0,a.Z)(e,s);return(0,i.mdx)("wrapper",(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,i.mdx)("h4",{id:"type-selectorstoreupdater"},"Type ",(0,i.mdx)("inlineCode",{parentName:"h4"},"SelectorStoreUpdater")),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},"A function with signature ",(0,i.mdx)("inlineCode",{parentName:"li"},"(store: RecordSourceSelectorProxy, data) => void")),(0,i.mdx)("li",{parentName:"ul"},"This interface allows you to ",(0,i.mdx)("em",{parentName:"li"},"imperatively")," write and read data directly to and from the Relay store. This means that you have full control over how to update the store in response to the subscription payload: you can ",(0,i.mdx)("em",{parentName:"li"},"create entirely new records"),", or ",(0,i.mdx)("em",{parentName:"li"},"update or delete existing ones"),". The full API for reading and writing to the Relay store is available ",(0,i.mdx)("a",{href:(0,o.default)("docs/api-reference/store/#recordsourceselectorproxy")},"here"),".")))}m.isMDXComponent=!0}}]);