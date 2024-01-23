# Project
The end goal of this project is to provide a PHP extension with bindings to the [C2PA Rust SDK](https://github.com/contentauth/c2pa-rs). I got started on this and was moved to a more expeditious project of a [PHP Wrapper](https://github.com/jrglasgow/c2pa-php) around the [c2patool (command line binary)](https://github.com/contentauth/c2patool).

I may have a chance to get back to this in the future.


## Current functionality
### Get PHP Extension version
```
C2PA::phpExtensionVersion()
```
will return the version of the PHP Extension
### Get Rust SDK version
```
C2PA::rustLibraryVersion()
```
will return the version of the Rust library that is included in the PHP extension.

### Create C2PA object
```
$c2pa = new C2PA()
```

### Add Signing Certificate location
```
cert_path = '/var/www/html/certs/es256_certs.pem';
$c2pa->setCertificate($cert_path);
```


### Add Signing Key location
```
$key_path = '/var/www/html/certs/es256_private.key';
$c2pa->setKey($key_path);
```


## To Build
Follow the installation instruction at https://davidcole1340.github.io/ext-php-rs/getting-started/installation.html to install Rust and the PHP development environment.

Ensure that the following was installed:
* Rust/Cargo
  * in Ubuntu `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  * exit terminal and re-enter or reload your PATH to get the location pf the binaries
* php-dev/php-config
  * in Ubuntu `sudo apt install php-dev`
* Clang
  * in Ubuntu `sudo apt install clang -y`
* libclang
  * in Ubuntu `sudo apt install libclang1 -y`
