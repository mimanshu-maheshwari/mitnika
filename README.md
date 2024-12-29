# Secrets Manager for Personal Projects:

## Problem Statement:
- When we are working on multiple project which have multiple versions and environments. But we are using a standard name for those secrets in our app. So we create multiple of these secrets files and different versions and environments with different names on our local, for running our project in local.
- What if we can select the correct environment file and enable it on click of a button or may be through CLI?
- Main idea is to manage all the files for projects, environments, versions and more using a simple app.

## Solution:
### Basic use case:
- Save file as secrets.
- When use wants to use the secret file, they select file, the file gets created on path if doesn't exists or the data is different.
- if the data is updated in the files i.e. the time-stamp of file is greater than that we have in app then confirm from user which file to use.
- Inform user we moved from which file to which file.
- We should be able to edit the file of any format(mostly).
  - Better if we are able to edit specific key value pair.
  - also if we can convert file from one format to other as well. (like save data as JSON or YAML and return values in format required like JSON, YAML, properties, etc)
- If we can store variables on fly as well that acts as placeholder.

## Actions:
### User can create/remove/modify collections/project:
### User can create/remove/modify files:
### User can create/remove/modify environments:
### User can create/remove/modify versions:
### User can create/remove/modify Global Environments Variables:
### User should be able to backup all data somewhere in local only

## Structure: 
Project -> File -> Environment -> Version
