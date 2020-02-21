## Bot Link
https://discordapp.com/oauth2/authorize?&client_id=679070237701832726&scope=bot&permissions=75776

# Feature List
* [X] Roll a Die: `/r ndm`, e.g. `/r 1d20`
* [X] Roll a Die (Default is `1d20`): `/r `
* [X] Roll Explain: `/rx {cmd}` tells you why you got what you got
* [X] Rolls Plus: `/r 1d20 + 3`
* [ ] Additive Rolls: `/r 1d20 + 1d20`
* [ ] Advantage Roll: `/ra ndm` e.g. `/ra 2d20`
  * `/ra 3d20` will pick the best outcome from 3 rolls
  * `/ra 1d20` is same as `/ra 2d20` because that's probably what you wanted. 
* [ ] Advantage Roll (Default is `/ra 2d20`) `/ra `

### Mid Term
* [ ] Roll with tagging: `/r #initiative`
* [ ] Tell Me What You Rolled: `/r 1d20 = 13 #initiative`

### Long Term
* [ ] Conditional Expression Roll: `/r ndm {comparison} k ? idj`
  * `/r 1d20 >= 13 ? 2d6`
* [ ] Multi-Conditional Expression Roll:
    * `/r 1d20 > 20,13 ? 4d6,2d6`
        * roll a `1d20`, if greater than 20 then roll a crit, if greater than 13 roll normal damage
* [ ] Roll History: `/rh {args}`
    * `/rh 10 @player1` last 10 rolls for player1
    * `/rh 10 #initiative` last 10 rolls tagged as an initiative roll
* [ ] Named Roll: `/r $billy-attacks = 1d20 + 5`
    * `/r $billy-attacks` show latest value of `billy-attacks`
    * `/r $billy-attacks!` re-perform the roll assigned to `billy-attacks`
