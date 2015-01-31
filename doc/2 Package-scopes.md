## Global packages
Global packages are maintained by upm.
If you developed your package, you can simply give it to upm (speak: make global), and it will copy and hold it safe for you. You don't have to care where to put the package, and you can simply delete your package directory, if you want, upm still has the copy. If you install a package from a server, you don't have to worry about the location, just install it globally and let upm handle all the dirty stuff for ye. It may also compress packages to save space.  
However, you can not edit global packages directly. That's why there are local packages:

## Local packages
Local packages are maintained by you, thus you can edit them and move them just as you like. It's not the fault of upm when your local package gets suddenly deleted or moved! You can still call most upm commands on local packages. However, if you want to distribute them, you have to make them global.

## Usage
If you want to develop a package, create a local package, edit it, debug it, and so on. If you're done and want it to be safe, give it to upm. If you want to edit a global package, just ask upm to copy it to a local package.  
If you want to download and use a package, install it globally. You don't care what stuff is in there.