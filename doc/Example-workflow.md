### The packager
1. `$ mkdir zip-gui`
1. Create `.upm-package-meta`
```
$ cat zip-gui/.upm-package-meta
name: zip-gui
version: 0.1.0
description: gui for zipping files with libzip
contact: example.com
dep.run = libzip@9-11
```
1. Create build, test and run scripts
1. Code the code
1. `$ upm verify ./zip-gui` (+correcting), `$ upm run ./zip-gui` (+debugging)
1. `$ upm copy ./zip-gui google.com:zip-gui` (if the packager owns Google)

### The user
1. `$ upm get zip-gui`
1. `$ upm run zip-gui`  

Or, if you want to run directly: 
 
1. `$ upm install zip-gui`
1. `$ zip-gui`  

The second method, however, is not safe, because you don't know whether the executable name is not `run-zip-gui` or something, you have to rely on the documentation in this moment.