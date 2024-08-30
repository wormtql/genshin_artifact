"use strict";(self["webpackChunkgenshin_artifacts"]=self["webpackChunkgenshin_artifacts"]||[]).push([[225],{49811:function(e,t,a){a.d(t,{n:function(){return o}});a(33948);async function n(){const e=await Promise.all([a.e(128),a.e(484),a.e(916)]).then(a.bind(a,29128));return e}const r=n();async function o(){return await r}},28396:function(e,t,a){a.d(t,{Z:function(){return s}});var n=a(10311),r=a(29491),o=(a(50657),a(60096),a(33948),a(64055)),l={__name:"SelectPreset",props:{modelValue:String},setup(e){const t=(0,o.B)(),a=(0,n.computed)((()=>{let e=[];for(let a of t.allFlat.value)e.push(a.name);return e}));return(t,o)=>{const l=r.BT,c=r.km;return(0,n.openBlock)(),(0,n.createBlock)(c,{"model-value":e.modelValue,"onUpdate:modelValue":o[0]||(o[0]=e=>t.$emit("update:modelValue",e))},{default:(0,n.withCtx)((()=>[((0,n.openBlock)(!0),(0,n.createElementBlock)(n.Fragment,null,(0,n.renderList)((0,n.unref)(a),(e=>((0,n.openBlock)(),(0,n.createBlock)(l,{key:e,label:e,value:e},null,8,["label","value"])))),128))])),_:1},8,["model-value"])}}};const c=l;var s=c},31652:function(e,t,a){a.r(t),a.d(t,{default:function(){return b}});var n=a(10311),r=a(89804),o=a(37336),l=(a(48504),a(20837),a(77049)),c=(a(75915),a(33948),a(49811)),s=a(28396),u=a(64055),i=(a(60285),a(41637),a(59e3));function d(e,t=12e4){const n=new Worker(new URL(a.p+a.u(380),a.b)),r=(0,i.p$)(e);return new Promise(((e,a)=>{const o=setTimeout((()=>{a("计算超时")}),t);n.onmessage=t=>{if("ready"===t.data.type)n.postMessage({args:[r],dispatch:"best_artifact_set"});else if("result"===t.data.type){const a=t.data.result;clearTimeout(o),e(a)}},n.onerror=e=>{a("计算发生错误："+e.message),clearTimeout(o)}})).finally((()=>{n.terminate()}))}var p=a(3857),f=a(5746);const m={class:"root"},y=["src"],g=["src"],v=["src"],T=["src"];var V=(0,n.defineComponent)({__name:"ArtifactSetTypeDisplay",props:{setType:null},setup(e){const t=e,a=(0,n.computed)((()=>{let e=[];if("string"===typeof t.setType)return[];"Set2"in t.setType?e.push(t.setType["Set2"]):"Set4"in t.setType?e.push(t.setType["Set4"]):"Set22"in t.setType&&(e.push(t.setType["Set22"][0]),e.push(t.setType["Set22"][1]));let a=[];for(let t of e){let e=(0,p.y)(t);const n=(0,f.dd)(e);a.push(n)}return a})),r=(0,n.computed)((()=>"string"!==typeof t.setType&&"Set2"in t.setType)),o=(0,n.computed)((()=>"string"!==typeof t.setType&&"Set4"in t.setType)),l=(0,n.computed)((()=>"string"!==typeof t.setType&&"Set22"in t.setType)),c=(0,n.computed)((()=>"Misc"===t.setType));return(e,t)=>((0,n.openBlock)(),(0,n.createElementBlock)("div",m,[(0,n.unref)(c)?((0,n.openBlock)(),(0,n.createElementBlock)(n.Fragment,{key:0},[(0,n.createTextVNode)("散件")],64)):(0,n.unref)(r)?((0,n.openBlock)(),(0,n.createElementBlock)(n.Fragment,{key:1},[(0,n.createElementVNode)("img",{src:(0,n.unref)(a)[0],class:"image"},null,8,y),(0,n.createTextVNode)("×2 ")],64)):(0,n.unref)(o)?((0,n.openBlock)(),(0,n.createElementBlock)(n.Fragment,{key:2},[(0,n.createElementVNode)("img",{src:(0,n.unref)(a)[0],class:"image"},null,8,g),(0,n.createTextVNode)("×4 ")],64)):(0,n.unref)(l)?((0,n.openBlock)(),(0,n.createElementBlock)(n.Fragment,{key:3},[(0,n.createElementVNode)("img",{src:(0,n.unref)(a)[0],class:"image"},null,8,v),(0,n.createElementVNode)("img",{src:(0,n.unref)(a)[1],class:"image"},null,8,T)],64)):(0,n.createCommentVNode)("",!0)]))}}),k=a(83744);const h=(0,k.Z)(V,[["__scopeId","data-v-eae5713a"]]);var w=h,N=a(9012);const _={class:"root"},x={class:"top"},B={class:"right"};var S=(0,n.defineComponent)({__name:"CalcBestArtifactSetPage",async setup(e){let t,a;const{t:i}=(0,N.QT)(),f=([t,a]=(0,n.withAsyncContext)((()=>(0,c.n)())),t=await t,a(),(0,u.B)()),m=(0,n.ref)(""),y=(0,n.computed)((()=>{if(""!==m.value)return f.getPreset(m.value)})),g=(0,n.computed)((()=>{var e;const t=null===(e=y.value)||void 0===e?void 0:e.item;if(!t)return null;let a=null;"custom"===t.artifactEffectMode&&(a=t.artifactConfig);const n={character:t.character,weapon:t.weapon,target_function:t.targetFunction,buffs:t.buffs,enemy:null,artifact_config:a};return n})),v=(0,n.ref)([]);async function T(){const e=r.kN.service({lock:!0,fullscreen:!0,text:"莫娜占卜中"}),t=await d(g.value,12e4);t.reverse();const a=t[0].value,n=[];for(let r=0;r<t.length;r++){const e=t[r];n.push({stats:e.stats,value:e.value,setType:e.set_type,ratio:e.value/a})}v.value=n,e.close()}function V(e){const t=(0,p.EO)(e);return i("stat",t)}return(e,t)=>{const a=l.mi,r=o.$Y,c=o.eI;return(0,n.openBlock)(),(0,n.createElementBlock)("div",_,[(0,n.createElementVNode)("div",x,[(0,n.createVNode)((0,n.unref)(s.Z),{modelValue:m.value,"onUpdate:modelValue":t[0]||(t[0]=e=>m.value=e),style:{"margin-right":"8px"}},null,8,["modelValue"]),(0,n.createVNode)(a,{onClick:T,type:"primary"},{default:(0,n.withCtx)((()=>[(0,n.createTextVNode)("Calc")])),_:1})]),(0,n.createElementVNode)("div",B,[(0,n.createVNode)(c,{data:v.value},{default:(0,n.withCtx)((()=>[(0,n.createVNode)(r,{label:"序号",width:"64"},{default:(0,n.withCtx)((({$index:e})=>[(0,n.createTextVNode)((0,n.toDisplayString)(e+1),1)])),_:1}),(0,n.createVNode)(r,{label:"套装","min-width":"120"},{default:(0,n.withCtx)((({row:e})=>[(0,n.createVNode)(w,{setType:e.setType},null,8,["setType"])])),_:1}),(0,n.createVNode)(r,{label:"时之沙"},{default:(0,n.withCtx)((({row:e})=>[(0,n.createTextVNode)((0,n.toDisplayString)(V(e.stats[0])),1)])),_:1}),(0,n.createVNode)(r,{label:"空之杯"},{default:(0,n.withCtx)((({row:e})=>[(0,n.createTextVNode)((0,n.toDisplayString)(V(e.stats[1])),1)])),_:1}),(0,n.createVNode)(r,{label:"理之冠"},{default:(0,n.withCtx)((({row:e})=>[(0,n.createTextVNode)((0,n.toDisplayString)(V(e.stats[2])),1)])),_:1}),(0,n.createVNode)(r,{label:"值/百分数"},{default:(0,n.withCtx)((({row:e})=>[(0,n.createTextVNode)((0,n.toDisplayString)(e.value.toFixed(1))+"/"+(0,n.toDisplayString)((100*e.ratio).toFixed(1))+"% ",1)])),_:1})])),_:1},8,["data"])])])}}});const C=(0,k.Z)(S,[["__scopeId","data-v-f27624ba"]]);var b=C},13171:function(e,t,a){a(3392)}}]);