"use strict";(self.webpackChunk=self.webpackChunk||[]).push([[7512],{3905:(e,n,t)=>{t.r(n),t.d(n,{MDXContext:()=>u,MDXProvider:()=>c,mdx:()=>y,useMDXComponents:()=>m,withMDXComponents:()=>d});var r=t(67294);function a(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function i(){return i=Object.assign||function(e){for(var n=1;n<arguments.length;n++){var t=arguments[n];for(var r in t)Object.prototype.hasOwnProperty.call(t,r)&&(e[r]=t[r])}return e},i.apply(this,arguments)}function o(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);n&&(r=r.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,r)}return t}function s(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?o(Object(t),!0).forEach((function(n){a(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):o(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function l(e,n){if(null==e)return{};var t,r,a=function(e,n){if(null==e)return{};var t,r,a={},i=Object.keys(e);for(r=0;r<i.length;r++)t=i[r],n.indexOf(t)>=0||(a[t]=e[t]);return a}(e,n);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)t=i[r],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(a[t]=e[t])}return a}var u=r.createContext({}),d=function(e){return function(n){var t=m(n.components);return r.createElement(e,i({},n,{components:t}))}},m=function(e){var n=r.useContext(u),t=n;return e&&(t="function"==typeof e?e(n):s(s({},n),e)),t},c=function(e){var n=m(e.components);return r.createElement(u.Provider,{value:n},e.children)},p={inlineCode:"code",wrapper:function(e){var n=e.children;return r.createElement(r.Fragment,{},n)}},f=r.forwardRef((function(e,n){var t=e.components,a=e.mdxType,i=e.originalType,o=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),d=m(t),c=a,f=d["".concat(o,".").concat(c)]||d[c]||p[c]||i;return t?r.createElement(f,s(s({ref:n},u),{},{components:t})):r.createElement(f,s({ref:n},u))}));function y(e,n){var t=arguments,a=n&&n.mdxType;if("string"==typeof e||a){var i=t.length,o=new Array(i);o[0]=f;var s={};for(var l in n)hasOwnProperty.call(n,l)&&(s[l]=n[l]);s.originalType=e,s.mdxType="string"==typeof e?e:a,o[1]=s;for(var u=2;u<i;u++)o[u]=t[u];return r.createElement.apply(null,o)}return r.createElement.apply(null,t)}f.displayName="MDXCreateElement"},61945:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>d,contentTitle:()=>l,default:()=>p,frontMatter:()=>s,metadata:()=>u,toc:()=>m});var r=t(83117),a=t(80102),i=(t(67294),t(3905)),o=["components"],s={id:"runtime-functions",title:"Runtime Functions",slug:"/api-reference/relay-resolvers/runtime-functions/",description:"Runtime functions associated with Relay Resolvers"},l=void 0,u={unversionedId:"api-reference/relay-resolvers/runtime-functions",id:"version-v17.0.0/api-reference/relay-resolvers/runtime-functions",title:"Runtime Functions",description:"Runtime functions associated with Relay Resolvers",source:"@site/versioned_docs/version-v17.0.0/api-reference/relay-resolvers/runtime-functions.md",sourceDirName:"api-reference/relay-resolvers",slug:"/api-reference/relay-resolvers/runtime-functions/",permalink:"/docs/api-reference/relay-resolvers/runtime-functions/",draft:!1,editUrl:"https://github.com/facebook/relay/tree/main/website/versioned_docs/version-v17.0.0/api-reference/relay-resolvers/runtime-functions.md",tags:[],version:"v17.0.0",frontMatter:{id:"runtime-functions",title:"Runtime Functions",slug:"/api-reference/relay-resolvers/runtime-functions/",description:"Runtime functions associated with Relay Resolvers"},sidebar:"docs",previous:{title:"Docblock Format",permalink:"/docs/api-reference/relay-resolvers/docblock-format/"},next:{title:"GraphQL Directives",permalink:"/docs/api-reference/graphql-and-directives/"}},d={},m=[{value:"LiveResolverStore",id:"liveresolverstore",level:2},{value:"<code>readFragment()</code>",id:"readfragment",level:2},{value:"<code>suspenseSentinel()</code>",id:"suspensesentinel",level:2},{value:"<code>useClientQuery()</code>",id:"useclientquery",level:2}],c={toc:m};function p(e){var n=e.components,t=(0,a.Z)(e,o);return(0,i.mdx)("wrapper",(0,r.Z)({},c,t,{components:n,mdxType:"MDXLayout"}),(0,i.mdx)("p",null,"This page documents the runtime functions associated with Relay Resolvers. For an overview of Relay Resolvers and how to think about them, see the ",(0,i.mdx)("a",{parentName:"p",href:"/docs/guides/relay-resolvers/introduction"},"Relay Resolvers")," guide."),(0,i.mdx)("h2",{id:"liveresolverstore"},"LiveResolverStore"),(0,i.mdx)("p",null,"To use Relay Resolvers you must use our experimental Relay Store implementation ",(0,i.mdx)("inlineCode",{parentName:"p"},"LiveResolverStore")," imported from ",(0,i.mdx)("inlineCode",{parentName:"p"},"relay-runtime/lib/store/experimental-live-resolvers/LiveResolverStore"),". It behaves identically to the default Relay Store but also supports Relay Resolvers."),(0,i.mdx)("p",null,"It exposes one additional user-facing method ",(0,i.mdx)("inlineCode",{parentName:"p"},"batchLiveStateUpdates()"),". See ",(0,i.mdx)("a",{parentName:"p",href:"/docs/guides/relay-resolvers/live-fields/#batching"},"Live Fields")," for more details of how to use this method."),(0,i.mdx)("h2",{id:"readfragment"},(0,i.mdx)("inlineCode",{parentName:"h2"},"readFragment()")),(0,i.mdx)("p",null,"Derived resolver fields model data that is derived from other data in the graph. To read the data that a derived field depends on, they must use the ",(0,i.mdx)("inlineCode",{parentName:"p"},"readFragment()")," function which is exported from ",(0,i.mdx)("inlineCode",{parentName:"p"},"relay-runtime"),". This function accepts a GraphQL fragment and a fragment key, and returns the data for the fragment."),(0,i.mdx)("admonition",{type:"warning"},(0,i.mdx)("p",{parentName:"admonition"},(0,i.mdx)("inlineCode",{parentName:"p"},"readFragment()")," may only be used in Relay Resolvers. It will throw an error if used in any other context.")),(0,i.mdx)("pre",null,(0,i.mdx)("code",{parentName:"pre",className:"language-tsx"},'import {readFragment} from "relay-runtime";\n\n/**\n * @RelayResolver User.fullName: String\n * @rootFragment UserFullNameFragment\n */\nexport function fullName(key: UserFullNameFragment$key): string {\n  const user = readFragment(graphql`\n    fragment UserFullNameFragment on User {\n      firstName\n      lastName\n    }\n  `, key);\n  return `${user.firstName} ${user.lastName}`;\n}\n')),(0,i.mdx)("p",null,"Note that Relay will ensure your field resolver is recomputed any time data in that fragment changes."),(0,i.mdx)("p",null,"See the ",(0,i.mdx)("a",{parentName:"p",href:"/docs/guides/relay-resolvers/derived-fields/"},"Derived Fields")," guide for more information."),(0,i.mdx)("h2",{id:"suspensesentinel"},(0,i.mdx)("inlineCode",{parentName:"h2"},"suspenseSentinel()")),(0,i.mdx)("p",null,"Live resolvers model client state that can change over time. If at some point during that field's lifecycle, the data being read is in a pending state, for example if the data is being fetched from an API, the resolver may return the ",(0,i.mdx)("inlineCode",{parentName:"p"},"suspenseSentinel()")," to indicate that the data is not yet available."),(0,i.mdx)("p",null,"Relay expects that when the data is available, the ",(0,i.mdx)("inlineCode",{parentName:"p"},"LiveStateValue")," will notify Relay by calling the subscribe callback."),(0,i.mdx)("pre",null,(0,i.mdx)("code",{parentName:"pre",className:"language-tsx"},"import {suspenseSentinel} from 'relay-runtime';\n\n/**\n * @RelayResolver Query.myIp: String\n * @live\n */\nexport function myIp(): LiveState<string> {\n  return {\n    read: () => {\n      const state = store.getState();\n      const ipLoadObject = state.ip;\n      if (ipLoadObject.status === \"LOADING\") {\n        return suspenseSentinel();\n      }\n      return state.ip;\n    },\n    subscribe: (callback) => {\n      return store.subscribe(callback);\n    },\n  };\n}\n")),(0,i.mdx)("p",null,"See the ",(0,i.mdx)("a",{parentName:"p",href:"/docs/guides/relay-resolvers/live-fields/"},"Live Fields")," guide for more information."),(0,i.mdx)("h2",{id:"useclientquery"},(0,i.mdx)("inlineCode",{parentName:"h2"},"useClientQuery()")),(0,i.mdx)("p",null,"If a query contains only client fields, it may not currently be used with hooks like ",(0,i.mdx)("inlineCode",{parentName:"p"},"usePreloadedQuery")," and ",(0,i.mdx)("inlineCode",{parentName:"p"},"useLazyLoadQuery")," since both of those hooks assume they will need to issue a network request. If you attempt to use these APIs in Flow you will get a type error."),(0,i.mdx)("p",null,"Instead, for client-only queries, you can use the ",(0,i.mdx)("inlineCode",{parentName:"p"},"useClientQuery")," hook:"),(0,i.mdx)("pre",null,(0,i.mdx)("code",{parentName:"pre",className:"language-tsx"},"import {useClientQuery} from 'react-relay';\n\nexport function MyComponent() {\n  const data = useClientQuery(graphql`\n    query MyQuery {\n      myIp\n    }\n  `);\n  return <div>{data.myIp}</div>;\n}\n")))}p.isMDXComponent=!0}}]);