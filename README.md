# next-butler

### File creation with the `new` command
```next-butler new {page|style|component} FILE_NAME```

Creates a new page, style or component file in it's corresponding folder.  
Eg.: `next-butler new page /users/index` creates the file index.js in the folder /pages/users

The functions inside the `page` and `component` files will be written with CamelCase. For example, if you name your component **super-comp**, **super_comp** or **superComp**, the function will be **SuperComp**

> To define what extension you want the file to have or any other configuration, you must specify it in the [config file](#config-file)

### What about the /src folder in Next?
If you have the `src` folder in the root directory, all the files created with the `new` command will be created inside that folder

### Config file
The config file, `nextbutler.json`, must be in the root file and contain objects defining the custom behaviour of the commands.  
Below are all the posible options with it's corresponding default value:

    {
        new: {
            page: {
                typescript: false // Create files as ts files?
                use_jsx: false // Create files as .tsx or .jsx
            },
            style: {
                ext: "css" // Can be scss, sass, or any other
            },
            component {
                typescript: false // Create files as ts files
                use_jsx: false // Create files as .tsx or .jsx
                folder: "components" // Where the components should be created
            }
        }
    }