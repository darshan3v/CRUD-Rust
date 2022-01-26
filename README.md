# CRUD-Rust
A Educational project that allows people to do CRUD operations on their text files

### Clone the Git Repo

Write the commands in directory where you want to clone it

```
git remote add origin git@github.com:darshan3v/ID_Nest.git
git clone git@github.com:darshan3v/ID_Nest.git
```
All the git commands should be written inside ID_Nest folder
### Git Commands

```
git add filename.extension      /// to add a file to a list which will get commited in your future commits

git add . /// all files will be added

git commit -m "describe what changes you made from previous commit in brief"

git push origin branch_name /// here branch name is the name of branch you want to push to remote repo

                          /// main branch name is master 


```
### Initial one-time git setup

```
git pull --rebase /// everytime before editing your code
                 ///everytime before you push 
                /// it just syncs your history of current branch with your remote repo,so no harm in using it everytime but must and should use in above 2 cases
                
Don't simply always push only  
when a significant(not in size but in terms of like if ur work is to do all design files then you can push for each HTML done 
                              OR 
When someone is waiting for your code , because there code depends on it 
                              AND
Sometimes in a frequency just so that others the amount of work done by you

git checkout branch_name    /// to swtich to the specified branch
```

### Git Workflow
**Be Careful about branches**

**If merge conflicts ovccur  then don't try to resolve by yourself contact the one with whose code it's conflicting, then discuss and decide**

1. Pull--rebase files
2. work
3. Add files to be tracked
4. Do commit's in significant cases
5. Pull--rebase
6. Push to Remote Branch

