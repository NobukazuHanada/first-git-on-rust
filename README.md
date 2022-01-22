# First Git on Rust

First Git on Rust is reimplementation with rust in order to learn about rust, c and git. 

## Reference project

This project refer to the oldest commit (commit ID is e83c5163316f89bfbde7d9ab23ca2e25604af290) of  https://github.com/git/git. To do this, I did the following.

First, I cloned this repository.

```
> git clone https://github.com/git/git.git
```

I moved this current directory to this repository. Then, I looked at the commit log in reverse order to find the oldest commit, using the following command.

```
> cd git
> git log –-reverse
```

Finally, I checked out to the Commit ID I found.

``` 
> git checkout -b first-commit-git <orldest-git-commit-id>
```

## Other helpful references

I listed up helpful references.

### [Git Internal (English Document)](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain)
The early git has a deep relationship with what we are doing inside the operations used in the current git. The chapter on git internal in the Pro git book is very helpful in understanding the earliest versions of git.

### [RustでつくるGit入門(Japanese Document)](https://zenn.dev/uzimaru0000/books/impl-git-in-rust)

This is a book by someone else about implementing Git with Rust. I haven't read it yet, but perhaps it will be helpful.
