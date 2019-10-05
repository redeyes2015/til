# Today I (want to) learn

* [Git submodule](./git/submodule.md) details ...
    * [ ] What is the typical workflow?
    * [x] What are sub-commands of submodule?
    * [ ] What does they do?
    * [ ] Can I "pin" to specific version?
    * [ ] What happen when "submodule" got updated?

* HTML paste
    * Stackoverflow: https://stackoverflow.com/questions/6333814/how-does-the-paste-image-from-clipboard-functionality-work-in-gmail-and-google-c
    * [codepen](https://codepen.io/redeyes2015/pen/bGbydPX?editors=1011)
    * [MDN: paste event](https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event)
        > an editable context (for example, in a <textarea> or an element with contenteditable attribute set to true)
        * `window.onpaste` would not work
    * [MDN: clipboardData](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent/clipboardData)
        * `event.clipboardData.items` an array-like object containing [DataTransferItem](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem)
        * `DataTransferItem.kind`: `"string"` or `"file"`
        * `DataTransferItem.type`: typically a MIME type
        * `DataTransferItem.getAsFile`: get a `File` object
        * `DataTransferItem.getAsString`: get string via callback function ([MDN: DataTransferItem.getAsString](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsString))
    
* Go project setup + build
    * What's the recommanded project setup?
    * How does glide find nested dependencies?
    * GOPATH?

* Go module
    * Where is the version / dep metadata?
    * What does `package` mean?

* Emscripten / WebAssembly
    * How can I build a hello-world in wasm?
    * How can I reduce the overhead of interlude?
    * Can I get rid of the node dependencies?

* Vuex
    * How to use it to extract out state-manipulating code?
    * How to write unit test for each parts?

* React + hook?
    * + Redux?

* VS Code + Refactor tools?
* "MPA"/traditional web framework?
    * RoR / Django / ?

* vim script?
* JS refactoring, patterns
* svelte
* Rust
* CA? / root CA? / certificate? / pem?

---

Brainstorm Question:

1. What new technology/concept would be certainly profitable in my day to day job in the next 3-6 months?
2. What field do I want to **deepen** my knowledge in?
3. What piece of technology/concept **excites** me and makes me want to try it?

Ref: https://dev.to/bgord/how-do-i-identify-my-knowledge-gaps-and-learn-4mlc
