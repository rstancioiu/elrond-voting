# Elrond voting
Elrond voting an example of a decentralised voting system on the Elrond blockchain.

----------

## Smart contract interface

### Create a voting poll
```
> erdpy call ... --function create  --arguments  ["str:vote", "str:EGLD to the moon ?", "u8:3|str:To the moon and back|str:To mars|str:To Papa Elon", "0x", "0x"]
```
This will automatically call the method *create* which creates a poll with:
- *name*: name of the poll ex.: "vote".
- *question*: question to ask ex.: "EGLD to the moon?".
- *choices*: the different choisces of the poll ex.:
    - "To the moon and back"
    - "To mars"
    - "To Papa Elon".
- *opt_deadline*: optional deadline in seconds till the end of the poll. By default it is one week.
- *opt_vote_limit*: optional vote limit when the poll will be finished.

### Voting to an alread created poll

```
> erdpy call ... --function vote --arguments["str:vote", "u32:0"]
```
This will enable one to vote for the poll named *vote* by choosing the first choice.

### Get results of the vote

```
> erdpy call ... --function get_results --arguments["str:vote"]
```
Returns the full resuls of the poll in case that the poll has been finished.

### Get Status

```
> erdpy call ... --function status --arguments["str:vote"]
```
Returns the status of the poll:
- *Archived*
- *Inactive*
- *Ended*
- *Running*

### Archived polls

```
> erdpy call ... --function get_poll_info_archived --arguments["str:vote"]
```
Once the poll has concluded it will be automatically added in the archived section of the smart contract where one can see the details of this poll.

For more details please feel free to check the mandos tests.

----------

## Configuration of the smart contract in local environment

### Install
To configure the environment, install elrond sdk. Then:
```
> manage.py --install
```

### Build
```
> manage.py --build
```

### Run tests
```
> manage.py --test
```
----------

You find any issues don't hesitate to submit a question via the Issues tab. They will be answered ASAP.