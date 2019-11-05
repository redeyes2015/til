# 參考

* [和 CORS 跟 cookie 打交道 by 愷開](https://medium.com/d-d-mag/%E5%92%8C-cors-%E8%B7%9F-cookie-%E6%89%93%E4%BA%A4%E9%81%93-dd420ccc7399)
* https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
* https://web.dev/cross-origin-resource-sharing/

## TL;DR:

* (browser side) 必須在 XHR 設定 `withCredentials` 或是 fetch 的選項中設置 `{ credentials: 'include' }`
* 視狀況 server 可能需要實作 preflight (回應 HTTP OPTION 請求)
* HTTP response header 要加上 `Access-Control-Allow-Credentials: true`
* 以及 `Access-Control-Allow-Origin` 不能使用 wildcard (`*`)
* 不能使用 [SameSite](https://web.dev/samesite-cookies-explained/)

# 如果可以掛在同一個 domain

例如: aaa.example.com 和 bbb.example.com

* 可以使用 `;Domain=example.com` (see domain [directive]) 於是在兩邊都看得到這個 cookie，
    也可以在 service 向 rd-service 發 CORS 並同時設定 cookie
* 如是要透過 CORS 設定 cookie，還是要設定 `Access-Control-Allow-Origin` 和 `Allow-Credentials` 以及 `withCredentials`
* 可以用 `SameSite`

[directive]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#Directives

## public suffix

但是，browser 會拒絕設在 "public suffix" 的 cookie ([ref][public-suffix]) 以避免安全漏洞，除了 top-level domain 如 .com 外，例如 EC2 共用的 domain: compute.amazonaws.com 也會註冊為 public suffix，避免不同用戶彼此影響 (也就是說不能拿 EC2 public DNS 作實驗)

[public-suffix]: https://tools.ietf.org/html/rfc6265#section-5.3

# Browser side 設定 (withCredentials)
## sample code
```javascript
// jquery
$.ajax(
  'http://another.host.example/login',
  {
    method: 'POST',
    xhrFields: {
     withCredentials: true
    }
  }
)
.then(
  () => console.log('success'),
  () => console.log('failed')
);

// fetch ; https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
fetch(
  'http://another.host.example/login',
  {
    method: 'POST',
    credentials: 'include'
  }
)
.then(
  () => console.log('success'),
  () => console.log('failed')
)
```
