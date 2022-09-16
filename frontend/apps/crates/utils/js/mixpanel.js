import Mixpanel from 'mixpanel-browser';
// Weird hacky solution. But I can't get wasm_bindgen to define a static_method_of Mixpanel without it
// also attempting to create an import like `import {Mixpanel} from '...'`.
//
// So this just exports Mixpanel in a way that makes everyone happy.
export {Mixpanel}
