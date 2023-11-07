# next-butler

##### DISCLAIMER
This is my first uploaded rust project. I'll try to improve it's readability

## Instalation
```cargo install next-butler```

## File creation with the `new` command
```next-butler new {page|style|component} FILE_NAME```

Creates a new page, style or component file in it's corresponding folder.  
Eg.: `next-butler new page /users/index` creates the file index.jsx in the folder /pages/users

The functions inside the `page` and `component` files will be written with PascalCase. 
For example, if you name your component **super-comp**, **super_comp** or **superComp**, the function will be **SuperComp**

> To define what extension you want the file to have or any other configuration, you must specify it in the [config file](#config-file)

## What about the /src folder in Next?
If you have the `src` folder in the root directory, all the files created with the `new` command will be created inside that folder

## Custom templates
You can define your own custom templates by creating them inside
`nextbutler/templates/[pages|stylesheets|components]/` with the extension you
want them to have.

For example, you can create a custom stylesheet template by creating the file
`my-super-stylesheet.scss` inside `nextbutler/templates/stylesheets/`, and
then use it with this syntax:
`nextbutler new style super-styles --template my-super-stylesheet.scss`. (Note:  
the template extension can be omitted if there is not another template with the  
same name)

## Config file
The config file, `nextbutler.json`, must be in the `nextbutler/` folder and
contain objects defining the custom behaviour of the commands.  
Below are all the posible options with it's corresponding default value:

    {
        new: {
            page: {
                typescript: false // Create files as .ts files?
                jsx: true // Create files as .tsx or .jsx
                template: "name" // Which template to use by default,
                page_router: false // Create page based on the old page router
            },
            style: {
                extension: 'css' // If the file has to have the .scss extension
                template: '' // Which template to use by default 
            },
            component {
                typescript: false // Create files as .ts files
                jsx: false // Create files as .tsx or .jsx
                folder: "components" // Where the components should be created
                template: '' // Which template to use by default 
            }
        }
    }

