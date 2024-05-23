# next-butler

## Instalation
```cargo install next-butler```

## File creation with the `new` command
```nb new [page|style|component] <name>```

Creates a new page, style or component file in it's corresponding folder.  
Eg.: `nb new page /users/index` creates the file index.jsx in the folder /pages/users

> [!NOTE]
> The router where pages will be created can be defined using the `--app-router` or
`--page-router` options, or defining it in the [config file](#config-file)

> [!NOTE]
> To define what extension you want the file to have or any other configuration, you
must specify it in the [config file](#config-file)

## Custom templates
You can define your own custom templates by creating them inside
`nextbutler/templates/[pages|stylesheets|components]/`.
The file extension can also be set by the template, by following this naming:
`<name>[.<extension>].hbs`

#### Example of a page template
```typescript
// nextbutler/templates/pages/example_tmpl.tsx.hbs

export default function {{ name }}() {
    return <h1>Welcome!</h1>
}
```
This template file will produce the following file when running (notice we omit the .hbs extension):
```
nb new page /home-page --template example_tmpl.tsx
```
```typescript
export default function HomePage() {
    return <h1>Welcome!</h1>
}
```

> [!NOTE]
> You can omit the extension when running the command if there is not another
template with the same name

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

## Initial configuration
You can create all the configuration files (with the default setup) running this command:
```
nb init
```
This will do the following:
- Create the tool's directory (nextbutler) inside the root dir
- Create the configuration file (nextbutler/nextbutler.json)
- Create the default templates as custom ones
