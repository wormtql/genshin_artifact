(function(){var n={33948:function(n,t,e){var r=e(17854),o=e(48324),i=e(98509),c=e(66992),u=e(68880),f=e(5112),a=f("iterator"),s=f("toStringTag"),p=c.values,l=function(n,t){if(n){if(n[a]!==p)try{u(n,a,p)}catch(r){n[a]=p}if(n[s]||u(n,s,t),o[t])for(var e in c)if(n[e]!==c[e])try{u(n,e,c[e])}catch(r){n[e]=c[e]}}};for(var b in o)l(r[b]&&r[b].prototype,b);l(i,"DOMTokenList")},3165:function(n,t,e){async function r(){const n=await Promise.all([e.e(128),e.e(484),e.e(320)]).then(e.bind(e,29128));self.onmessage=function(t){const e=t.data.potentialFunctionInterface,r=t.data.artifacts,o=n.PotentialInterface.get_potential(r,e);self.postMessage({type:"results",data:{results:o}})},self.postMessage({type:"ready"})}e(33948),r().catch((n=>{console.log("error from compute potential worker: "+n.toString())}))}},t={};function e(r){var o=t[r];if(void 0!==o)return o.exports;var i=t[r]={id:r,exports:{}};return n[r](i,i.exports,e),i.exports}e.m=n,e.x=function(){var n=e.O(void 0,[48],(function(){return e(3165)}));return n=e.O(n),n},function(){var n="function"===typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",t="function"===typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",r="function"===typeof Symbol?Symbol("webpack error"):"__webpack_error__",o=function(n){n&&!n.d&&(n.d=1,n.forEach((function(n){n.r--})),n.forEach((function(n){n.r--?n.r++:n()})))},i=function(e){return e.map((function(e){if(null!==e&&"object"===typeof e){if(e[n])return e;if(e.then){var i=[];i.d=0,e.then((function(n){c[t]=n,o(i)}),(function(n){c[r]=n,o(i)}));var c={};return c[n]=function(n){n(i)},c}}var u={};return u[n]=function(){},u[t]=e,u}))};e.a=function(e,c,u){var f;u&&((f=[]).d=1),f&&(f.moduleId=e.id);var a,s,p,l=new Set,b=e.exports,d=new Promise((function(n,t){p=t,s=n}));d[t]=b,d[n]=function(n){f&&n(f),l.forEach(n),d["catch"]((function(){}))},d.moduleId=e.id,e.exports=d,c((function(e){var o;a=i(e);var c=function(){return a.map((function(n){if(n[r])throw n[r];return n[t]}))},u=new Promise((function(t){o=function(){t(c)},o.r=0;var e=function(n){n!==f&&!l.has(n)&&(l.add(n),n&&!n.d&&(o.r++,n.push(o)))};a.map((function(t){t[n](e)}))}));return o.r?u:c()}),(function(n){n?p(d[r]=n):s(b),o(f)})),f&&(f.d=0)}}(),function(){var n=[];e.O=function(t,r,o,i){if(!r){var c=1/0;for(s=0;s<n.length;s++){r=n[s][0],o=n[s][1],i=n[s][2];for(var u=!0,f=0;f<r.length;f++)(!1&i||c>=i)&&Object.keys(e.O).every((function(n){return e.O[n](r[f])}))?r.splice(f--,1):(u=!1,i<c&&(c=i));if(u){n.splice(s--,1);var a=o();void 0!==a&&(t=a)}}return t}i=i||0;for(var s=n.length;s>0&&n[s-1][2]>i;s--)n[s]=n[s-1];n[s]=[r,o,i]}}(),function(){e.d=function(n,t){for(var r in t)e.o(t,r)&&!e.o(n,r)&&Object.defineProperty(n,r,{enumerable:!0,get:t[r]})}}(),function(){e.f={},e.e=function(n){return Promise.all(Object.keys(e.f).reduce((function(t,r){return e.f[r](n,t),t}),[]))}}(),function(){e.u=function(n){return"js/"+n+"-legacy."+{48:"fd1cfc7e",128:"4aa58ca3",320:"2d801487",484:"330c12b5"}[n]+".js"}}(),function(){e.miniCssF=function(n){}}(),function(){e.g=function(){if("object"===typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(n){if("object"===typeof window)return window}}()}(),function(){e.o=function(n,t){return Object.prototype.hasOwnProperty.call(n,t)}}(),function(){e.r=function(n){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(n,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(n,"__esModule",{value:!0})}}(),function(){e.v=function(n,t,r,o){var i=fetch(e.p+""+r+".module.wasm");return"function"===typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(i,o).then((function(t){return Object.assign(n,t.instance.exports)})):i.then((function(n){return n.arrayBuffer()})).then((function(n){return WebAssembly.instantiate(n,o)})).then((function(t){return Object.assign(n,t.instance.exports)}))}}(),function(){e.p="/"}(),function(){var n={165:1},t=function(t){var r=t[0],i=t[1],c=t[2];for(var u in i)e.o(i,u)&&(e.m[u]=i[u]);c&&c(e);while(r.length)n[r.pop()]=1;o(t)};e.f.i=function(t,r){n[t]||importScripts(e.p+e.u(t))};var r=self["webpackChunkgenshin_artifacts"]=self["webpackChunkgenshin_artifacts"]||[],o=r.push.bind(r);r.push=t}(),function(){var n=e.x;e.x=function(){return e.e(48).then(n)}}();e.x()})();