# Temporary Package directories

## Motivation  
If the package needs a directory outside of itself for temporary files,
there is no way of getting one.  
If the package needs a temporary directory for intermediate build files,
which it can reuse if the temporary directory still exists for speeding things up,
there is no way to find one but in the package itself.  
However, some packagers would rather like to have an outside-of-source build.  
And if the temporary files are inside the package, upm will not be able to clean
them if the user wishes to clean space.  
For example: where do object files go when compiling C?
Put it into `tmp` inside the package. Okay.  
Now we can reuse them the next time we build if the source of the object file did not change.  
But some packagers would rather have an outside-of-source build so they dont have to worry
about the temp dir and to keep the package clean.

## Short and understandable summary
upm should have a command that, given a package directory, will return a temporary
directory which is associated to that directory. The next call with the same directory
will return the same temp dir because it belongs to that package directory.  
Then there is another command that deletes all temporary directories to clear space.
This is safe because only intermediate files go there.  
For example:  
Have a package located at /opt/clang .  
Now the command will, given that path, always return /usr/local/share/upm/clang-tmp or something else.

## Advantages
 * Dont let the packager worry about temporary directories
 * Faster building because of reusing intermediate files because they can be put in tmp dirs
 * Easy and safe way of cleaning temporary files (upm does not go into package to delete)
 * Keep the source package clean, not tainted with build files
 * outside-of-source builds are possible, which packagers may want

## Disadvantages
No known

## Alternatives  
 * Have the package create the temporary directories inside the package.  
-> upm can`t delete files directly bc that would be unsafe
-> package tainted with build files
-> limits packagager if he actually wants outside-of-source build
 * Have the packager create the temporary directories outside itself.  
-> This will make the packager more work, because it has to be portable.  
-> On Unix, temporary files allocated by OS will be deleted after relogin -> intermediate files have to be regenerated -> slower  

## Unresolved questions
All the stuff that is still to be discussed and decided, or other holes in your proposal.  

## Detailed explanation
upm will put all temporary directories in one directory, which depends on the platform.  
The names of the temporary directories are the paths of their respective packages compressed without data loss.
The command to get the temp dir is `upm gettmp <directory>`, the command to clear all tmps is `upm clean [<pacakge>]`
