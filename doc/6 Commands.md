Commands
--------

**`run <package-identifier> <args>`**  

Executes the run script of the package matching `package-identifier` if a program.  

***

**`install [--no-get] <package-identifier> [<path>]`**  

Executes the install script of the package, passes `<path>` if it is given.
Will also `test` the package if possible.
Will `get` if the package is not found on your computer unless `--no-get` is specified.

**`uninstall <package-identifier>`**
Executes the uninstall script of the package, which will uninstall the **global installation**.
To "uninstall" a local install, just delete the directory, idiot!
Problem: confusion with `remove`

***

**`server add <server>`**
Adds a server to the registered servers.

***

**`server remove <server>`**
Removes a server to the registered servers.

***

**`server list`**
Lists all registered servers.

***

**`search [--first | --one] [--all] [<servers>] <package-identifier>`**

Search the given servers or all registered servers (if no server is given) for the global `package-identifier` and print all matching packages.  
If `--first` is given, it will only print the first matching packages.  
If `--one` is given, it will fail if there is not exactly one matching package.  
Unless `--all` is given, it will print only the highest version for each package name.  

***

**`get [pass to _search_] <package-identifier>`**  

`search`es for the package and `copy`s the result.

Fails if no package is returned by `search` and lets you choose if multiple packages are returned.

***

**`remove [(-l | --local)] <package-identifier>`**  

Removes the package matching `package-identifier`.  
Will fail if package is local but neither of `-l` or `--local` is given.  
Will fail if package is global but `-l` or `--local` is given.  
After that, upm will check if the package *is* a upm package using `verify`.

***

**`link <package-ident-from> [<package-ident-from2> [...]] <package-ident-to>`**  
Creates a package `<package-ident-to>`, which acts like a normal package but links to one or more packages.


***

**`test <package-identifier>`**  

Runs the test script of the package matching `package-identifier`.  
Returns whether the package has been built properly and whether it is able to run.  
If no test script is given, it succeeds.

***

**`verify <package-identifier>`**  

Tests whether this is a valid package. In other words, whether you can call any command on this package (not to confuse with `test`, which tests if a package is runnable).  
Additionally, it runs the verify script of the package, if it exists.  
`load`s if no meta file exists.

***

**`copy [(-f | --force)] <package-ident-from> <package-ident-to>`**  

Copies the package matching `package-ident-from` so that the second copy matches `package-ident-to`.  
If `package-ident-to` has a matching package, the command will only execute when either `-f` or `--force` is listed.
If `package-ident-to` is not complete, the omitted information will be taken by `package-ident-from`.  
For example: `copy gcc@4.8 @4.9` will copy to `gcc@4.9`  
Will `load` the new package at `<package-ident-to>` e.g. the newly created package.

Use to make local packages global or copy packages between archives.  
For example: `copy clang ./` will copy clang to `./clang`, e.g. right in front of you :smile:.

***

**`download ...`**  
Does the same as `copy` but ensures that the package will be copied from a foreign server to this computer.

***

**`upload ...`**  
Does the same as `copy` but ensures that the package will be copied from this computer to a foreign server.


***

**`move [(-f | --force)] <package-ident-from> <package-ident-to>`**  

Same effect as `copy` and `remove [--local] <package-ident-from>` afterwards.

***

**`load <package-identifier>`**  

Will run the main script of the package matching `package-identifier`, if any.  
That means it updates the meta data of the package.

***

**`install-pkg-dep <package-identifier>`**  

Will `install` all package dependencies of the package matching `package-identifier` to their respective directories.

***

**`install-run-dep <package-identifier>`**

Will `install` all runtime dependencies of the package matching `package-identifier`  to their respective directories.

***
