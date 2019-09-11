<a name="v0.3.0"></a>
## v0.3.0 (2019-09-11)


#### Styles

*   Apply rustfmt ([f29ef579](f29ef579))

#### Features

*   Add error handler ([2fb4ed8f](2fb4ed8f))
*   Add debug logging ([dc0405bb](dc0405bb))
* **MavenCoordinates:**
  *  Implement Display trait ([078ab402](078ab402))
  *  Add constructor method ([b8b59e2e](b8b59e2e))

#### Refactorings

*   Improve error handling for checksum fetching ([bc32ce99](bc32ce99))
*   Extract main function into separate ones ([cc50b98f](cc50b98f))
*   Lazily evaluate fallback function calls ([63b8bf54](63b8bf54))

#### Builds

*   Use workaround for missing strum EnumVariantNames ([4c2d7354](4c2d7354))
*   Use strum v0.16.0 from refactor branch ([77781253](77781253))



<a name="v0.2.0"></a>
## v0.2.0 (2019-05-22)


#### Bug Fixes

*   Support unorthodox Maven coordinates ([9300ae09](9300ae09))

#### Features

*   Read raven version from Cargo.toml ([3ca869b6](3ca869b6))



<a name="v0.1.1"></a>
## v0.1.1 (2019-05-22)


#### Features

*   Support overriding Maven repository & checksum hash algorithm ([0326676c](0326676c))

#### Documentation

*   Generate new changelog for v0.1.0 using custom Clog config ([bea09c1f](bea09c1f))



<a name="v0.1.1"></a>
## v0.1.1 (2019-05-22)


#### Features

*   Support overriding Maven repository & checksum hash algorithm ([0326676c](0326676c))



<a name="v0.1.0"></a>
## v0.1.0 (2019-05-18)


#### Features

*   Implement checksum fetching for Maven artifacts (#1) ([8a130ff2](8a130ff2))

