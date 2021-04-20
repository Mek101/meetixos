# MeetiX Operating System syscall

In this document are described the functions that compose the system calls provided by the kernel.

## **Open syscall**

Initializes a reference to an existing filesystem resource or creates a new one.

```c
int open(char *path, int flags, ...);
```

#### Parameters:

* ```path```:  An absolute or relative path to the resource to open/create
* ```flags```: A list of bitwise combined values from the following constants
	* ```O_READ```:
	* ```O_WRITE```:
	* ```O_EXEC```:
	* ```O_TRUNC```:
	* ```O_CREAT```:
	* ```O_EXCL```:
	* ```O_AT```:
	* ```O_REG```:
	* ```O_DIR```:
	* ```O_CHAN```:
	* ```O_LINK```:
	* ```O_READ```: Opens the resource for reading
	* ```O_WRITE```: Opens the resource for writing
	* ```O_EXEC```: Opens the resource for execution
	* ```O_CREAT```: Creates a new resource accoding to the third parameter
	* ```O_EXCL```: Opens the resource exclusively, fails if already opened, any other open by other processes will fail
	* ```O_NDID```: Opens the node by its node number: works only on links
	* ```O_REG```: Fails if the resource is not a regular file
	* ```O_DIR```: Fails if the resource is not a directory entry
	* ```O_CHAN```: Fails if the resource is not a special channel file
	* ```O_LINK```: Fails if the resource is not a link resource
* ```...```: Must be filled with a ```mode``` when ```O_CREAT``` is provided, then the resource will be created according to that

#### Returns:

A non negative integer value that references the filesystem resource opened. A negative value on failure.

#### Errors:

* ```ENOENT```:
* ```ENOMEM```:
* ```ENOTREG```:
* ```ENOTDIR```:
* ```ENOTCHAN```:
* ```ENOTLINK```:
* ```EROFS```:
* ```EPERM```:
* ```EBUSY```:

## Close syscall

Dereferences a filesystem resource from the caller process

```c
int close(int fd);
```

#### Parameters:

* ```fd```: The integer that refers the resource to dereference

## Remove syscall

Dereferences a filesystem resource from the filesystem tree.<br>
So in case of any other references, and right permissions, the resource is deleted.

```c
int remove(int fd);
```

#### Parameters:

* ```fd```: The integer that refers the resource to dereference

## Read syscall

Reads content bytes from a filesystem resource.

```c
ssize_t read(int fd, void *buffer, size_t len);
```

#### Parameters:

* ```fd```: The integer that refers the resource to read
* ```buffer```: Pointer to the buffer where the kernel stores the readed bytes
* ```len```: Amount of bytes to read


```c
// [ Function Prototype ] write -------------------------------------------------- //
ssize_t write(int fd, void *buffer, size_t len);

// [ Function Prototype ] rwstat ------------------------------------------------- //
int rwmeta(int fd, int rw, struct meta *meta, int mask);

// [ Function Prototype ] forkf -------------------------------------------------- //
int forkf(int flags);

// [ Function Prototype ] exits -------------------------------------------------- //
void exits(int code);

// [ Function Prototype ] addreg ------------------------------------------------- //
int addreg(void **addr, size_t length);

// [ Function Prototype ] mount -------------------------------------------------- //
int mount(int fd, const char *mntp, int flags);

// [ Function Prototype ] fdctl -------------------------------------------------- //
int fdctl(int fd, int cmd, void *arg);

```
