# coding=utf-8
import requests
from apig_sdk import signer

if __name__ == '__main__':
    sig = signer.Signer()
    # Set the AK/SK to sign and authenticate the request.
    sig.Key = "WYVNLLAN4NP7BXVSSBDE"
    sig.Secret = "6YExQmUha0jgM0td6GFG46Z4pMxSgXWeHrJA0Hk1"

    # The following example shows how to set the request URL and parameters to query a VPC list.
    # Set request Endpoint.
    # Specify a request method, such as GET, PUT, POST, DELETE, HEAD, and PATCH.
    # Set request URI.
    # Set parameters for the request URL.
    r = signer.HttpRequest("GET", "https://vpc.cn-southwest-2.myhuaweicloud.com/v1/de0114f9034240d298ea6187124bd35f/vpcs?limit=1")
    # Add header parameters, for example, x-domain-id for invoking a global service and x-project-id for invoking a project-level service.
    r.headers = {"content-type": "application/json"}
    # Add a body if you have specified the PUT or POST method. Special characters, such as the double quotation mark ("), contained in the body must be escaped.
    r.body = ""
    sig.Sign(r)
    print(r.headers["X-Sdk-Date"])
    print(r.headers["Authorization"])
    resp = requests.request(r.method, r.scheme + "://" + r.host + r.uri, headers=r.headers, data=r.body)
    print(resp.status_code, resp.reason)
    print(resp.content)
