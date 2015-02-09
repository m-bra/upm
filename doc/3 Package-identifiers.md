Every time upm has to know about a package you provide it an **identifier**. The problem is, that, given one identifier, upm has to know which exact package you meant, which can be difficult if there are many packages.

## Identifiers for loose packages
Loose identifiers are easy. They are just a path to your package, either absolute or relative.  
If you have a relative path, begin your path with `./`.
If you have an absolute path, begin your path with `/` on Unix or `X:` on Windows (with X being the disk).

## Identifiers for maintained packages
Each package has a **server**, **group**, a **name**, and a **version**.  
An identifier consists of the name and optionally the server, group and/or version of the package.  
There are full identifiers which give all the information, which can only match exactly one or no package *in the entire freaking world* (or the entire server if server is omitted).  
If you leave out some information, multiple package can fit to the identifier. Some commands like `upm copy` will infer missing information.  
The syntax of an identifier is: `[<servers>:][<group>/]<name>[<version>]`.  
`<servers>` is a comma separated list of the possible servers the matching package is in, but is usually just one server (if not omitted).

## Filters
Filters are used to specify a range of values which can be a bit complex. They only let in some values, and reject others.

### Wildcards
Wildcards are one kind of filter.  
A wildcard consists of *conditions* which are *logically combined*.  

Possible conditions are:
 * `*` - let any value through
 * `=x` or `x` - let only values equal to x through
 * `!x` - let only values not equal to x through
 * `<x` `>x` `<=x` `>=x` - let only values which are bigger/less than and optionally equal to x through

Possible combinations are:
 * `a&b` - let value only through if it fits through the conditions a and b.
 * `a|b` - ~ a or b
 * `a^b` - ~ a or b, but not both  

You *can* put braces around conditions for readibility.

Examples:  
`(>1)|=2` - lets values bigger than one or equal to two through  
`(<2)` - lets values smaller than two through  
`>=3&!4` - lets values bigger or equal than three but four through  
`1` - lets only values equal to one through  

## Versions
A version consists of **numbers** and **flags**.  

The numbers can also have letters as digits, where numbers have a higher value than letters. They are separated by dots.  At least one number must be there.  

The flags are just strings separated by hyphens.  

Versions start with an `@`

For example:  
`@12.5.a`  
`@12`  
`@A.1.0`  
`@1.2.b-debug`  
`@1.3-unfree-debug`

### Versions as filters

Versions are also filters. They only let versions through which have the same beginning of the numbers list and the same beginning of the flags list.  
If you want the version filter to match versions which have *exactly* the same numbers list, end the numbers with a dot. End the flags list with an underscore to achieve the same effect.  

For example:  
`@12.5.a` => `12.5`: match (same beginning)  
`@12` => `13.5`: no match  
`@A.1.0` => `A.1.`: no match (numbers must match exactly here)  
`@1.2.b-debug` => `1.2.b.-debug`: match (numbers match exactly)  
`@1.3-unfree-debug` => `1.3.-unfree-debug-`: match  

Versions as filters have a more generic form; they are wildcards which let versions through as values. That means they can be combined (like `12.5|11.1`) and do not have to start with `@` since wildcards dont either.  

### Numbers and flags as filters
Also, one number or flag of a version can also act as a filter (Technically, they are always filters, including filters which only fit with one exact value).

That's why this works:  
`1.1.*`  
`1.0.>2`  
`A.(<4)-!debug`

### Belonging of combinings and conditions
Now, consider following version filter:  
`12.3|13.5`  
Where does the `|` belong to? To the whole versions or the numbers?  

To clarify the statement, do either `(12.3)|(13.5)` or `12.(3|13).5`. See why this is a problem now? By default, the combinings (and conditions!) belong to the whole version if the belonging is ambiguous.
