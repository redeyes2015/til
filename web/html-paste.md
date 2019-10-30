
# HTML paste

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

