# Outside Auras

Proof of concept for wow overlays using live log data

![Showcaseimg](https://i.imgur.com/wEOlkJp.png)

## Why

Personally I have known this was possible for a long time, but actually thought the delay would have been too bad. But after seeing that WarcraftLogs was going to do an overlay using the logdata(https://twitter.com/WarcraftLogs/status/1686485331346948098). And testing what the delay actually was in a raid group. I felt I needed to shed some light on the potential road we are heading towards


## How does it work

By reading the combatlog that wow saves to the disk, over the years the delay between the writes have gone down. And now in a 20 player raid we are looking at an average delay of 300ms from the ingame event to seeing it on the disk, which make it almost imperceptible. 

## Showcase

See the video at https://youtu.be/VvQ7O4N8rtk heroic neltharion. The "last delay" shows the delay between our reads of the logfiles. Under this you will see the Volcanic Heart aura pop up, happens first time at 00:19. Note that there is almost no perceptible delay between the outside aura and the actual game.

Again this is just a showcase I hope no-one uses this for actual progression

## Consequences

The combatlog that's written to the disk contains a lot of information that's not normally available to ingame addons/weakauras due to them being abused over they years. Some notable examples are private auras(as I'm showcasing with the neltharion aura), player positions(example of an old aura that could be possible again: https://youtu.be/Vx6ipbVOWvY?t=220). In general just take a look at the replay feature from WarcraftLogs, and then imagine it running as an overlay with 300 ms delay.

There are also some ways of reducing the delay which I won't get in to here. But it would most likely be doable to create a version with around 50ms delay

## How can it be fixed
Only flush the logs to the disk after the encounter is over

## How do I run it?

Please don't use it for any difficult content only for verifying that it works, and that it should not be allowed, there is a replay feature you can use if you want to see how it would have looked in your own kill.

```
outside-auras.exe PATH_TO_LOG replay
```

## Why even release it?

Showing is better than telling and creating this is trivial. It's literally just watching a file being written to. If I didn't then somebody else would have made some private version of this, when it's revealed that it's possible. Or worse somebody would make an Overwolf app


## I don't trust your exe how can I build it myself

Good never trust random strangers, install cargo pull the repo `cargo build`. If the window is invisible you might have to apply the bug fix manually from https://github.com/emilk/egui/issues/2537
