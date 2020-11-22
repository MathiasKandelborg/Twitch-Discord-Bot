# TD BOT

A work-in-progress bot


## Features

- Native notifications with images
- Run commands on the computer 

## APIs in use
- Twitch Web sockets
- _to be implemented_ - Discord

## Instructions

1. Create Discord application
2. Authorize application [at this link](https://discord.com/oauth2/authorize?client_id=<CLIENT_ID>&scope=bot&permissions=8) `<CLIENT_ID>` & changing permissions to what you want/need


The app uses configuration files:

- settings 
- commands

It also reads sensitive settings from ENV.

| Key           | Value                                       | Description                                   |
|    ---           |                             ---                |          ---                                     |
| T_AUTH_TOKEN  | Token from https://twitchapps.com/tmi/      | Chat token, user who redeemed is the username |
| T_OAUTH_TOKEN | Token from https://twitchapps.com/tokengen/ | Generated with developer app id and scopes    |
| T_CHANNEL_ID  | The _unique_ channel/user ID                | Found with the twitch API                     |

It supports several file formats (TBA)
The settings should be provided as key/value pairs


# Licensing

See LICENSE
