"use strict";(self.webpackChunk=self.webpackChunk||[]).push([[9439],{3905:(e,n,t)=>{t.r(n),t.d(n,{MDXContext:()=>d,MDXProvider:()=>u,mdx:()=>g,useMDXComponents:()=>s,withMDXComponents:()=>c});var a=t(67294);function r(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function i(){return i=Object.assign||function(e){for(var n=1;n<arguments.length;n++){var t=arguments[n];for(var a in t)Object.prototype.hasOwnProperty.call(t,a)&&(e[a]=t[a])}return e},i.apply(this,arguments)}function o(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);n&&(a=a.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,a)}return t}function l(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?o(Object(t),!0).forEach((function(n){r(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):o(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function m(e,n){if(null==e)return{};var t,a,r=function(e,n){if(null==e)return{};var t,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)t=i[a],n.indexOf(t)>=0||(r[t]=e[t]);return r}(e,n);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)t=i[a],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(r[t]=e[t])}return r}var d=a.createContext({}),c=function(e){return function(n){var t=s(n.components);return a.createElement(e,i({},n,{components:t}))}},s=function(e){var n=a.useContext(d),t=n;return e&&(t="function"==typeof e?e(n):l(l({},n),e)),t},u=function(e){var n=s(e.components);return a.createElement(d.Provider,{value:n},e.children)},p={inlineCode:"code",wrapper:function(e){var n=e.children;return a.createElement(a.Fragment,{},n)}},f=a.forwardRef((function(e,n){var t=e.components,r=e.mdxType,i=e.originalType,o=e.parentName,d=m(e,["components","mdxType","originalType","parentName"]),c=s(t),u=r,f=c["".concat(o,".").concat(u)]||c[u]||p[u]||i;return t?a.createElement(f,l(l({ref:n},d),{},{components:t})):a.createElement(f,l({ref:n},d))}));function g(e,n){var t=arguments,r=n&&n.mdxType;if("string"==typeof e||r){var i=t.length,o=new Array(i);o[0]=f;var l={};for(var m in n)hasOwnProperty.call(n,m)&&(l[m]=n[m]);l.originalType=e,l.mdxType="string"==typeof e?e:r,o[1]=l;for(var d=2;d<i;d++)o[d]=t[d];return a.createElement.apply(null,o)}return a.createElement.apply(null,t)}f.displayName="MDXCreateElement"},68629:(e,n,t)=>{t.d(n,{Z:()=>p});var a=t(39960),r=t(86341),i=t(67294);function o(){var e=window.encodeURI(JSON.stringify({title:"Feedback about "+window.location.pathname,description:"**!!! Required !!!**\n\nPlease modify the task description to let us know how the docs can be improved.\n\n**Please do not ask support questions via this form! Instead, ask in fburl.com/relay_support**",tag_ids:{add:[0xac96423e5b680,0x64079768ac750]}}));window.open("https://www.internalfb.com/tasks/?n="+e)}function l(e){var n=e.children;return i.createElement("div",{className:"docsRating",id:"docsRating"},i.createElement("hr",null),n)}var m=function(){var e=i.useState(!1),n=e[0],t=e[1],a=function(e){t(!0),function(e){window.ga&&window.ga("send",{hitType:"event",eventCategory:"button",eventAction:"feedback",eventValue:e})}(e)};return n?"Thank you for letting us know!":i.createElement(i.Fragment,null,"Is this page useful?",i.createElement("svg",{className:"i_thumbsup",alt:"Like",id:"docsRating-like",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 81.13 89.76",onClick:function(){return a(1)}},i.createElement("path",{d:"M22.9 6a18.57 18.57 0 002.67 8.4 25.72 25.72 0 008.65 7.66c3.86 2 8.67 7.13 13.51 11 3.86 3.11 8.57 7.11 11.54 8.45s13.59.26 14.64 1.17c1.88 1.63 1.55 9-.11 15.25-1.61 5.86-5.96 10.55-6.48 16.86-.4 4.83-2.7 4.88-10.93 4.88h-1.35c-3.82 0-8.24 2.93-12.92 3.62a68 68 0 01-9.73.5c-3.57 0-7.86-.08-13.25-.08-3.56 0-4.71-1.83-4.71-4.48h8.42a3.51 3.51 0 000-7H12.28a2.89 2.89 0 01-2.88-2.88 1.91 1.91 0 01.77-1.78h16.46a3.51 3.51 0 000-7H12.29c-3.21 0-4.84-1.83-4.84-4a6.41 6.41 0 011.17-3.78h19.06a3.5 3.5 0 100-7H9.75A3.51 3.51 0 016 42.27a3.45 3.45 0 013.75-3.48h13.11c5.61 0 7.71-3 5.71-5.52-4.43-4.74-10.84-12.62-11-18.71-.15-6.51 2.6-7.83 5.36-8.56m0-6a6.18 6.18 0 00-1.53.2c-6.69 1.77-10 6.65-9.82 14.5.08 5.09 2.99 11.18 8.52 18.09H9.74a9.52 9.52 0 00-6.23 16.9 12.52 12.52 0 00-2.07 6.84 9.64 9.64 0 003.65 7.7 7.85 7.85 0 00-1.7 5.13 8.9 8.9 0 005.3 8.13 6 6 0 00-.26 1.76c0 6.37 4.2 10.48 10.71 10.48h13.25a73.75 73.75 0 0010.6-.56 35.89 35.89 0 007.58-2.18 17.83 17.83 0 014.48-1.34h1.35c4.69 0 7.79 0 10.5-1 3.85-1.44 6-4.59 6.41-9.38.2-2.46 1.42-4.85 2.84-7.62a41.3 41.3 0 003.42-8.13 48 48 0 001.59-10.79c.1-5.13-1-8.48-3.35-10.55-2.16-1.87-4.64-1.87-9.6-1.88a46.86 46.86 0 01-6.64-.29c-1.92-.94-5.72-4-8.51-6.3l-1.58-1.28c-1.6-1.3-3.27-2.79-4.87-4.23-3.33-3-6.47-5.79-9.61-7.45a20.2 20.2 0 01-6.43-5.53 12.44 12.44 0 01-1.72-5.36 6 6 0 00-6-5.86z"})),i.createElement("svg",{className:"i_thumbsdown",alt:"Dislike",id:"docsRating-dislike",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 81.13 89.76",onClick:function(){return a(0)}},i.createElement("path",{d:"M22.9 6a18.57 18.57 0 002.67 8.4 25.72 25.72 0 008.65 7.66c3.86 2 8.67 7.13 13.51 11 3.86 3.11 8.57 7.11 11.54 8.45s13.59.26 14.64 1.17c1.88 1.63 1.55 9-.11 15.25-1.61 5.86-5.96 10.55-6.48 16.86-.4 4.83-2.7 4.88-10.93 4.88h-1.35c-3.82 0-8.24 2.93-12.92 3.62a68 68 0 01-9.73.5c-3.57 0-7.86-.08-13.25-.08-3.56 0-4.71-1.83-4.71-4.48h8.42a3.51 3.51 0 000-7H12.28a2.89 2.89 0 01-2.88-2.88 1.91 1.91 0 01.77-1.78h16.46a3.51 3.51 0 000-7H12.29c-3.21 0-4.84-1.83-4.84-4a6.41 6.41 0 011.17-3.78h19.06a3.5 3.5 0 100-7H9.75A3.51 3.51 0 016 42.27a3.45 3.45 0 013.75-3.48h13.11c5.61 0 7.71-3 5.71-5.52-4.43-4.74-10.84-12.62-11-18.71-.15-6.51 2.6-7.83 5.36-8.56m0-6a6.18 6.18 0 00-1.53.2c-6.69 1.77-10 6.65-9.82 14.5.08 5.09 2.99 11.18 8.52 18.09H9.74a9.52 9.52 0 00-6.23 16.9 12.52 12.52 0 00-2.07 6.84 9.64 9.64 0 003.65 7.7 7.85 7.85 0 00-1.7 5.13 8.9 8.9 0 005.3 8.13 6 6 0 00-.26 1.76c0 6.37 4.2 10.48 10.71 10.48h13.25a73.75 73.75 0 0010.6-.56 35.89 35.89 0 007.58-2.18 17.83 17.83 0 014.48-1.34h1.35c4.69 0 7.79 0 10.5-1 3.85-1.44 6-4.59 6.41-9.38.2-2.46 1.42-4.85 2.84-7.62a41.3 41.3 0 003.42-8.13 48 48 0 001.59-10.79c.1-5.13-1-8.48-3.35-10.55-2.16-1.87-4.64-1.87-9.6-1.88a46.86 46.86 0 01-6.64-.29c-1.92-.94-5.72-4-8.51-6.3l-1.58-1.28c-1.6-1.3-3.27-2.79-4.87-4.23-3.33-3-6.47-5.79-9.61-7.45a20.2 20.2 0 01-6.43-5.53 12.44 12.44 0 01-1.72-5.36 6 6 0 00-6-5.86z"})))},d=function(){return i.createElement("p",null,"Let us know how these docs can be improved by",i.createElement("a",{className:"button",role:"button",tabIndex:0,onClick:o},"Filing a task"))},c=function(){return i.createElement("p",null,"Help us make the site even better by"," ",i.createElement(a.default,{to:"https://www.surveymonkey.com/r/FYC9TCJ"},"answering a few quick questions"),".")},s=function(){return i.createElement(l,null,i.createElement(d,null),i.createElement(m,null),i.createElement(c,null))},u=function(){return i.createElement(l,null,i.createElement(m,null),i.createElement(c,null))};const p=function(){return(0,r.fbContent)({internal:i.createElement(s,null),external:i.createElement(u,null)})}},25137:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>s,contentTitle:()=>d,default:()=>f,frontMatter:()=>m,metadata:()=>c,toc:()=>u});var a=t(83117),r=t(80102),i=(t(67294),t(3905)),o=t(68629),l=["components"],m={id:"wait-for-fragment-data",title:"waitForFragmentData",slug:"/api-reference/wait-for-fragment-data/",description:"Read the value of a fragment as a promise",keywords:["promise","fragment"]},d=void 0,c={unversionedId:"api-reference/relay-runtime/wait-for-fragment-data",id:"api-reference/relay-runtime/wait-for-fragment-data",title:"waitForFragmentData",description:"Read the value of a fragment as a promise",source:"@site/docs/api-reference/relay-runtime/wait-for-fragment-data.md",sourceDirName:"api-reference/relay-runtime",slug:"/api-reference/wait-for-fragment-data/",permalink:"/docs/next/api-reference/wait-for-fragment-data/",draft:!1,editUrl:"https://github.com/facebook/relay/tree/main/website/docs/api-reference/relay-runtime/wait-for-fragment-data.md",tags:[],version:"current",frontMatter:{id:"wait-for-fragment-data",title:"waitForFragmentData",slug:"/api-reference/wait-for-fragment-data/",description:"Read the value of a fragment as a promise",keywords:["promise","fragment"]},sidebar:"docs",previous:{title:"observeFragment",permalink:"/docs/next/api-reference/relay-runtime/api-reference/observe-fragment"},next:{title:"Runtime Configuration",permalink:"/docs/next/api-reference/runtime-config/"}},s={},u=[{value:"<code>waitForFragmentData</code>",id:"waitforfragmentdata",level:2},{value:"Example: Deferring data used in an event handler",id:"example-deferring-data-used-in-an-event-handler",level:3},{value:"Arguments",id:"arguments",level:3},{value:"Return Value",id:"return-value",level:3}],p={toc:u};function f(e){var n=e.components,t=(0,r.Z)(e,l);return(0,i.mdx)("wrapper",(0,a.Z)({},p,t,{components:n,mdxType:"MDXLayout"}),(0,i.mdx)("admonition",{type:"warning"},(0,i.mdx)("p",{parentName:"admonition"},(0,i.mdx)("inlineCode",{parentName:"p"},"waitForFragmentData")," is still an experimental API. It currently has some limitations and may evolve slightly during this phase.")),(0,i.mdx)("h2",{id:"waitforfragmentdata"},(0,i.mdx)("inlineCode",{parentName:"h2"},"waitForFragmentData")),(0,i.mdx)("p",null,"In some cases it can be useful to define data that you wish to read using a GraphQL fragment, but then consume it just once outside of React render function. ",(0,i.mdx)("inlineCode",{parentName:"p"},"waitForFragmentData")," allows you to wait for the data of a fragment to be avalaible,"),(0,i.mdx)("p",null,"To read a fragment's data as it changes over time, see ",(0,i.mdx)("a",{parentName:"p",href:"/docs/next/api-reference/relay-runtime/api-reference/observe-fragment"},(0,i.mdx)("inlineCode",{parentName:"a"},"observeFragment")),"."),(0,i.mdx)("h3",{id:"example-deferring-data-used-in-an-event-handler"},"Example: Deferring data used in an event handler"),(0,i.mdx)("p",null,"One use case for ",(0,i.mdx)("inlineCode",{parentName:"p"},"waitForFragmentData")," is to defer fetching data that is needed inside an event handler, but is not needed to render the initial view."),(0,i.mdx)("pre",null,(0,i.mdx)("code",{parentName:"pre",className:"language-tsx"},'import { useCallback } from "react";\nimport { useFragment } from "react-relay";\nimport { graphql } from "relay-runtime";\nimport { waitForFragmentData } from "relay-runtime/experimental";\n\nfunction MyComponent({ key }) {\n  const user = useFragment(\n    graphql`\n      fragment UserFragment on User {\n        name\n        # Page load can complete before this data has streamed in from the server.\n        ...EventHandlerFragment @defer\n      }\n    `,\n    key,\n  );\n\n  const onClick = useCallback(async () => {\n    // Once the user clicks, we may need to wait for the data to finish loading.\n    const userData = await waitForFragmentData(\n      graphql`\n        fragment EventHandlerFragment on User {\n          age\n        }\n      `,\n      user,\n    );\n\n    if (userData.age < 10) {\n      alert("Hello kiddo!");\n    } else if (userData.age < 18) {\n      alert("Hello young person!");\n    } else {\n      alert("Hello adult person!");\n    }\n  }, [user]);\n\n  return (\n    <div>\n      My name is {user.name}\n      <button onClick={onClick}>Greet</button>\n    </div>\n  );\n}\n')),(0,i.mdx)("h3",{id:"arguments"},"Arguments"),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"environment"),": ",(0,i.mdx)("inlineCode",{parentName:"li"},"IEnvironment"),". A Relay environment."),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"fragment"),": GraphQL fragment specified using a ",(0,i.mdx)("inlineCode",{parentName:"li"},"graphql")," template literal."),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("inlineCode",{parentName:"li"},"fragmentReference"),": The ",(0,i.mdx)("em",{parentName:"li"},"fragment reference")," is an opaque Relay object that Relay uses to read the data for the fragment from the store; more specifically, it contains information about which particular object instance the data should be read from.",(0,i.mdx)("ul",{parentName:"li"},(0,i.mdx)("li",{parentName:"ul"},"The type of the fragment reference can be imported from the generated Flow types, from the file ",(0,i.mdx)("inlineCode",{parentName:"li"},"<fragment_name>.graphql.js"),", and can be used to declare the type of your ",(0,i.mdx)("inlineCode",{parentName:"li"},"Props"),". The name of the fragment reference type will be: ",(0,i.mdx)("inlineCode",{parentName:"li"},"<fragment_name>$key"),". We use our ",(0,i.mdx)("a",{parentName:"li",href:"https://github.com/relayjs/eslint-plugin-relay"},"lint rule")," to enforce that the type of the fragment reference prop is correctly declared.")))),(0,i.mdx)("h3",{id:"return-value"},"Return Value"),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},"A ",(0,i.mdx)("inlineCode",{parentName:"li"},"Promise<T>")," where ",(0,i.mdx)("inlineCode",{parentName:"li"},"T")," is the data defined in the fragment.")),(0,i.mdx)("p",null,"The Promise will wait for all network data to become avaliable as well as any ",(0,i.mdx)("a",{parentName:"p",href:"/docs/next/guides/relay-resolvers/live-fields/"},(0,i.mdx)("inlineCode",{parentName:"a"},"@live")," Relay Resolver")," to be in a non-suspended state before it resolves."),(0,i.mdx)("p",null,"In the case of a network error, or a field-level error due to ",(0,i.mdx)("a",{parentName:"p",href:"/docs/next/guides/throw-on-field-error-directive/"},(0,i.mdx)("inlineCode",{parentName:"a"},"@throwOnFieldError"))," or ",(0,i.mdx)("a",{parentName:"p",href:"/docs/next/guides/required-directive/"},(0,i.mdx)("inlineCode",{parentName:"a"},"@required(action: THROW)")),", the Promise will reject with an error."),(0,i.mdx)(o.Z,{mdxType:"DocsRating"}))}f.isMDXComponent=!0}}]);