THE SOFTWARE IS PROVIDED "AS IS" AND BRIAN SMITH AND THE AUTHORS DISCLAIM
ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES
OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL BRIAN SMITH OR THE AUTHORS
BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY
DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.



ring-ffi
========

[*ring*](https://github.com/briansmith/ring) is a crypto library in Rust based
on BoringSSL's crypto primitive implementations.

ring-ffi is a wrapper around *ring* that packages it in a shared library with a
libffi-compatible API. The goal is to use ring-ffi as the foundation for
*ring* bindings to Ruby, Python, Node.js, etc.



Language Bindings Being Developed
---------------------------------

Language     | Bindings
------------ | --------
Ruby         | [ruby-ring](https://github.com/cryptosphere/ruby-ring)
Rust         | Just use the native [*ring*](https://github.com/briansmith/ring) API.
Python       | [cryptography-ring](https://github.com/reaperhulk/cryptography-ring)
 

Contributing
------------

Patches Welcome! Suggestions:

* Map more *ring* functionality to the libffi-compatible API.
* Improve the bindings for existing languages.
* Create bindings for new languages. (If you do so, please send a PR here to
  add your binding to the table above.



License
-------

See [LICENSE](LICENSE).



Bug Reporting
-------------

Please file bugs in the
[issue tracker](https://github.com/briansmith/ring-ffi/issues). Yes, even
security vulnerabilities should be reported in the public issue tracker. **Do
NOT report any security vulnerability privately to
the *ring-ffi* developers.**
