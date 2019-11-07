# 參考

* [和 CORS 跟 cookie 打交道 by 愷開](https://medium.com/d-d-mag/%E5%92%8C-cors-%E8%B7%9F-cookie-%E6%89%93%E4%BA%A4%E9%81%93-dd420ccc7399)
* https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
* https://web.dev/cross-origin-resource-sharing/

## TL;DR:

* (browser side) 必須在 XHR 設定 `withCredentials` 或是 fetch 的選項中設置 `{ credentials: 'include' }`
* 視狀況 server 可能需要實作 preflight (回應 HTTP OPTION 請求)
* HTTP response header 要加上 `Access-Control-Allow-Credentials: true`
* 以及 `Access-Control-Allow-Origin` 不能使用 wildcard (`*`)
* SameSite 檢定和 Cross-Site 檢定不同，通過 SameSite 檢定就可以使用 SameSite Directive
    * 並且瀏覽器似乎都會視為 first-party，即不會被視為第三方 cookie 而被阻擋
* 如果不用 SameSite，CORS 還是可以設定 cookie
    * 除非瀏覽器設定阻擋第三方 cookie 或是遇到 Firefox 的 [First party isolation][fpi]

[fpi]: https://www.ctrl.blog/entry/firefox-fpi.html

## Browser side sample code (withCredentials)
```javascript
// jQuery.ajax : https://api.jquery.com/jquery.ajax/
jQuery.ajax(
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

// fetch : https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
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

# SameSite

[SameSite Cookie，防止 CSRF 攻击 - 紫云飞](https://www.cnblogs.com/ziyunfei/p/5637945.html)

> 判断两个 URL 是不是同一个网站的，只要判断两个 URL 的域名的 public suffix（按能匹配到的最长的算）
> 以及它前面的那个字段（后面用 public suffix+1 指代）是否都相同，是的话就是同一个站点的，否则不是。

跟 cross origin 的判斷不一樣! 只要通過 SameSite 檢定，cross origin request 就可以使用 SameSite [directive]，
不過 cross origin request 需要的設定還是要加。

也就是，假設 .com 是 public suffix 但 example.com 不是，從 aaa.example.com，發到 bbb.example.com
的 request:

1. 會被視為 cross origin 因此需要設定請求端的 `withCredentials` 以及回應端的 `Access-Control-Allow-Origin` 和 `Access-Control-Allow-Credentials`
2. 但視為 SameSite，也就不會因使用者設定的「阻擋第三方 cookie」而失效，也可以使用 SameSite [directive]

## public suffix

除了用來判斷 SameSite，browser 會拒絕設在 "public suffix" 的 cookie ([ref][public-suffix]) 以避免安全漏洞，
除了 top-level domain 如 .com 外，例如 EC2 共用的 domain: compute.amazonaws.com 也會註冊為 public suffix，
避免不同用戶彼此影響 (也就是說不能拿 EC2 public DNS 作實驗)

[public-suffix]: https://tools.ietf.org/html/rfc6265#section-5.3

## Domain directive

可以使用 `;Domain=example.com` (see domain [directive]) 讓 aaa.example.com，發到 bbb.example.com 兩邊
都看得到這個 cookie，同時也有兩邊都會帶這個 cookie 的缺點，需要避免命名衝突；同樣名字，但是不同 directive 的
cookie 雖然可以同時存在，但是在 HTTP request header 裡只會出現一次，也就是會有「遮蓋」的效果。

[directive]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#Directives
