# next-butler

## Instalation
```cargo install next-butler```

## File creation with the `new` command
```next-butler new [page|style|component] <name>```

Creates a new page, style or component file in it's corresponding folder.  
Eg.: `next-butler new page /users/index` creates the file index.jsx in the folder /pages/users

> [!NOTE]
> The router where pages will be created can be defined using the `--app-router` or
`--page-router` options, or defining it in the [config file](#config-file)

> [!NOTE]
> To define what extension you want the file to have or any other configuration, you
must specify it in the [config file](#config-file)

## Custom templates
You can define your own custom templates by creating them inside
`nextbutler/templates/[pages|stylesheets|components]/`.

For example, you can create a custom stylesheet template by creating the file
`my-super-stylesheet` inside `nextbutler/templates/stylesheets/`, and
then use it with this syntax:
`next-butler new style super-styles --template my-super-stylesheet`.

## Config file
The config file, `nextbutler/nextbutler.json`, must contain objects defining
the custom behaviour of the commands.  
Below are all the possible options with it's corresponding default value:

    {
        new: {
            page: {
                typescript: false // Create files as .ts files?
                jsx: true // Create files as .tsx or .jsx
                template: null // Which template to use by default
                api_template: null // Which template to use by default for api routes
                page_router: false // Create page based on the old page router
            },
            style: {
                extension: 'css' // If the file has to have the .scss extension
                folder: "styles" // Where the stylesheets should be created
                template: null // Which template to use by default
            },
            component {
                typescript: false // Create files as .ts files
                jsx: true // Create files as .tsx or .jsx
                folder: "components" // Where the components should be created
                template: null // Which template to use by default 
            }
        }
    }

