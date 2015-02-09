## Maintained packages
Those packages are maintained by upm.
If you developed your package, you can simply give it to upm, and it will copy and hold it safe for you. You don't have to care where to put the package, and you can simply delete your package directory, if you want, upm still has the copy. If you install a package from a server, you don't have to worry about the location, just install as a maintained package and let upm handle all the dirty stuff for ye. It may also compress packages to save space.  
Also, it will store all versions you gave separately (automacically), so if you give it clang@2.2.1 and clang@3.2, it will save them both. If you saved them loosely, you had to make ugly directory names like clang_2_2_1 and clang_3_2.
However, you can not edit maintained packages directly. That's why there are free packages:

## Loose packages
Loose packages are maintained by you, thus you can edit them and move them just as you like. It's not the fault of upm when your loose package gets suddenly deleted or moved! You can still call most upm commands on loose packages.

## Usage
If you want to develop a package, create a loose package, edit it, debug it, and so on. If you're done and want it to be safe, give it to upm.  
If you want to edit a maintained package, just ask upm to copy it to a free package.  
If you want to download and use a package, install it as in maintained mode. You don't care what stuff is in there.
