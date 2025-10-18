
pub const DESCRIPTION_SHORT:    &'static str = "*a lily on a valley grown for software engineering, social causes and artistry*";
pub const DESCRIPTION_LONG_MD:     &'static str = r#"Hello, welcome to my tiny corner of the Internet! My projects, contributions, beliefs and ideas in [Open Source Software](https://en.wikipedia.org/wiki/Open-source_software) (or differential yet equally nerdy subjects) are showcased here.

Quite a lot of my personal life can be elided, with the most notable points being: I started my journey in digital art at a very young age. Around the same time, I was also curious about the sciences of computers and electronics down to their very fundamentals. Soon after, I learned to write batch scripts, wrote very rudimentary game mods and continued studying for the arts of such things up to today.

With interests in Computer Science, Anthropology and Psychology, I find value in the technologies and teachings of social causes and public benefit. Movements akin to the [Free Software Foundation](https://fsf.org) and the [Free Art License](http://artlibre.org/) (*published in Françias*; [available in English](https://artlibre.org/licence/lal/en/)) are some of my favorite references for sustaining an ethical, '*Copyleft*' attitude; inspiring optimism for a freer and fairer world where ingenuity, integrity and ethics prevails in technology for the betterment of Humanity.

My hobbies include music production (with synthesizers and a [Digitakt]()), writing fanfic and novels strictly for myself, CAD/3D printing, pencil art and calligraphy. I’m practicing more self-care too, which as it turns out, is very important for one’s wellbeing. There’s a lot of smaller things to boast, but… you can rightly imagine they’re very nerdy at this point so I’ll save you the words.

In ceasing my endless ramblings, I’d also like to take a moment to appreciate and thank-you for stopping by today! For questions or inquiries, [Contact](contact/) me.

₍^⸝⸝> ·̫ <⸝⸝^₎

#### My Fun Facts:
- 🥑 Fruitfully enjoying a good Avocado
- 🐧 Daily-driving GNU Linux (Ubuntu w/ Xen Hypervisor) and Mac OS.
  - 🙂‍↕️Yes, I know… Macs and Apple. I know. Worse yet, I have no justification for my behaviors and choices. I even have an iPhone… Flay me already, I’m guilty.
  - See also my [[Workstations & Software]] list
- 🟩 Playing survival Minecraft (Java & Bedrock)
- ☕️ Enjoying locally roasted coffees
- ⌨️ Building a 70% DIY RP2040-based Kailh Purple Mechanical Keyboard with home-brewed firmware
- 🛜 Using Espressif IDF and select ESP32 chips w/ HomeAssistant
- 🦊 A tail-wearing, ear-donning Furry (hi furs! 👋)
- ⚙️ Considers itself to be more of 'thing' than a Human being
-  🧠 Neurodivergent
- 🏳️‍🌈 Has many pride flags
  - 🩵🩷🤍🩷🩵 
  - 💙💛🩷 
  - 🖤🩶🤍💜 
  - 🩷💙♾️ 
- Also has CLI flags `-la` `-vv` `-h` 🤭

---
⭐️ My work and content is free, and I hope you find it useful! If you do, and you have the funds to spare, please consider supporting me:

[![GitHub Sponsors](https://img.shields.io/badge/GitHub%20Sponsors-Donate%20on%20GitHub%20Sponsors-181717?style=flat&logo=github&logoColor=white)](https://github.com/sponsors/lilyanavalley)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/L4L01DWUOJ)

*Of course, donations are not required, but they **are** very appreciated!*
"#;

pub const ASK_DESCRIPTION_MD: &'static str = "Questions about [Rust]()? Maybe, you (and other visitors) would like to know something oddly specific to Technology, Art, Copyleft and/or the Human Condition? Ask me anything, *Reddit AMA* style.

An account or social media platform is not required to ask me questions. All questions are anonymous, yet public.";
pub const ASK_ASKSIMILAR: &'static str = "Ask a Similar Question";
pub const ASK_REVIEW: &'static str = "Request a Review";

pub const WORK_MD: &'static str = "# Projects


### Peace / `peacelily`
📜 License `AGPL-3` | 🌐 [GitHub Repo](https://github.com/lilyanavalley/peace)

The website you’re currently browsing, based off of Rust web framework [Leptos](https://leptos.rs). It’s the successor to a previous personal website I worked on called `lavender`, made with [Rocket](https://rocket.rs) and pure HTML/CSS. This newer iteration focuses on the use of WASM and reactive web development.

Eventually, I’d like to transform Peace into a completely customizable profile/portfolio service that others can use for their own purposes.

----
### *Codename* Experiment A / `exa`
📜 License `GPL-3` | 🌐 [GitHub Repo](https://github.com/lilyanavalley/exa)

Using the Fyrox game engine, `exa` is an exploration of game development in Rust. Currently, the direction of the game and its gameplay loop is '*to be determined*' but its source code is available.

This particular project spawned off of an earlier alpha build on the concept of trading, marketing and corporate expansion; illustrating capitalistic values, exploits and the political corruption it fosters in an otherworldly society of Humans, Animalkin, Eldrich Horrors and Monsters. `exa` could hold the same tone, but I’m not quite sure where I want to go with it yet.

----
### *Codename* Experiment B / `exb`
 🤷 Unreleased

An experimental game, still yet to be determined.

----
### BoreGUI
(Pronounced *bore-gooey*)
📜 License `GPL-3` | 🌐 [GitHub Repo](https://github.com/lilyanavalley/boregui)

Graphical user interface for [Borg Backup](). I really like using Borg on my servers and wanted to extend its usage to desktop environments. Written in Rust utilizing the `egui` framework.

----
### `dots-rust-leptos`
📜 `Public Domain` | 🌐 [GitHub Repo](https://github.com/lilyanavalley/dots-rust-leptos)

Dot files which initialize a working Rust & Leptos development environment, particularly for my own use within GitHub Codespaces.

----
### `dots-workstation`
 🤷 Unreleased

Dot files for initializing and updating personal workstations with the apps, tools and libraries I’d need to feel comfy. This includes updated Python, Rust, C/C++ environments and the IDEs for them.

----
### *Codename* `keys`
 🤷 Unreleased

Pair of firmware and desktop application packages for DIY keyboards. Keys is an exploration of the RP2040 chip and embedded Rust development. Its accompanying desktop application offers customizable layouts, layers, macros, 'overlay HUD', plugin API and additionally writes its settings to the keyboard for portability.

Sadly, I’m not quite comfortable sharing this project until it’s proven stable enough not to brick hardware. 😅


----
## Contributions


### `conduwuit`
📜 License `Apache-2` | 🌐 [My Fork](https://github.com/lilyanavalley/conduwuit)
Matrix chat server, originally a [hard fork of conduit by girlbossceo (aka: June/Strawberry/puppygock)](https://github.com/girlbossceo/conduwuit) and archived after some time. Since I found it to be very cool, I’ve forked it and have been poking about with it.  Many thanks go to Strawberry and the project’s contributors!
";

pub const CONTACT_DESCRIPTION: &'static str = "Have a question or statement? Send me a direct message! An account or specific social media platform is not necessary to contact me.";
pub const CONTACT_INSTRUCTIONS_MD: &'static str = "Once you send a message, your browser will remember our conversation history. You will need to return periodically for updates. If your browser has privacy-centric features that delete Browser Cookies, you should also note the **SCID** to return to conversations in the future in case your browser doesn't do so automatically.";
pub const CONTACT_PGP_INSTRUCTIONS_MD: &'static str = "You may [encrypt messages](https://emailselfdefense.fsf.org/en/) using my public key:";
pub const PGPKEY: &'static str = "-----BEGIN PGP PUBLIC KEY BLOCK-----

mFMEZ8Mw9hMJKyQDAwIIAQEHAgMEIKYIB6z+9OlIfVFTnxMmWBTPPSRqc6VmtOhK
J+o/e2cCCcQ2TJPURKc1Oe6NU+qDLxHgzRD2hBdrJe+tD/GEs7QpTGlseSBBbmEg
VmFsbGV5IDxtc2xpbHl2YWxsZXlAaWNsb3VkLmNvbT6ImQQTEwgAQRYhBCCXBnXw
3QdimlciKTeIXPe545KYBQJnwzD2AhsDBQkFo5qABQsJCAcCAiICBhUKCQgLAgQW
AgMBAh4HAheAAAoJEDeIXPe545KYfxoBAIGeBVV+4fEmNztFDGsPmU/kWIAo3IxI
Xhu01wqmE9q2AP9I8oUAKCRP3sGc2D1zWW1Z8dDwy9nzae/5ODAroUAx6LhXBGfD
MPYSCSskAwMCCAEBBwIDBHQ7UbOounXaiD9a7iFVuIS+MAbjFyjbx1n1ORhqf1uy
BWTG9zhoMa6G+S/07/0+jiZVeqIo7bGyUfKns3WWnjoDAQgHiH4EGBMIACYWIQQg
lwZ18N0HYppXIik3iFz3ueOSmAUCZ8Mw9gIbDAUJBaOagAAKCRA3iFz3ueOSmAtG
AQCV+6KaiUqSnPCoyykwhf6KwB5UMONoJNyfd8pvyXsxmAEAm5YXPvc6WC71sykk
OnB4nbOV7ZZVvZCuAsI6FEb+/bs=
=8avv
-----END PGP PUBLIC KEY BLOCK-----";
pub const CONTACT_BE_NICE: &'static str = "Not all messages recieve replies. Please be respectful and treat me as you wish to be treated.";
pub const CONTACT_PRIVACY_REVEALER: &'static str = "Privacy & Confidentiality Statement";
pub const CONTACT_PRIVACY_SECT_CLOUD_MD: &'static str = "This portal relays messages to/from a Web Service operated by me via [DigitalOcean](#) in a Datacenter located at *Toronto, Canada*. Messages are passed through the public Internet using [HTTPS](#), [Application-Level Encryption](#), and [At-Rest Encryption](#).";
pub const CONTACT_PRIVACY_SECT_ANTISPAM_MD: &'static str = "On submission, connection metadata, browser session and city geolocation is recorded strictly for spam prevention. Once a conversation has ended *or* 72 hours elapses without further contact, [my server deletes our conversation history](). Backups are performed and retained for a maximum of one week with encrypted/hashed+salted forms of active correspondence.";
pub const CONTACT_PRIVACY_SECT_CLIENTSIDE_MD: &'static str = "Preventative measures such as these help to ensure confidentiality, but *do **not** absolutely guarantee confidentiality*. Unforeseen circumstances may expose correspondence; Anything from [phishing]() attempts to a peek over the shoulder... Especially where confidentiality is crucial, please practice safe, sane and secure computer use. And, if you notice anything incorrect, please file a complaint ASAP on the the [GitHub Issues page for this Web Service]().";

// ? Alternative contact page while the chat is being built.
pub const CONTACT_ALTERNATIVE: &'static str = "# Contact

To get in touch with me, you'll need a **Matrix** or **Telegram**[1] account.

📧 You may notice there's no email address here... and that's entirely on purpose. I don't trust email to reliably find
its way to my inbox, avoiding spam and snooping unless the average visitor is privy to PGP. If you still require my email address, please ask me over those channels.

### Matrix
Message [@lilflwr:catgirl.cloud](https://matrix.to/#/@lilflwr:catgirl.cloud) directly, or scan this QR code to see my profile:

```

    █▀▀▀▀▀█ ▀▄▄▀▀ ▀ ▀ █▄▄ █▀▀▀▀▀█    
    █ ███ █   ▀ ██ █▄▄ ██ █ ███ █    
    █ ▀▀▀ █  ▀▄▀▄▄▀ █▄▀▀  █ ▀▀▀ █    
    ▀▀▀▀▀▀▀ █ █▄▀▄▀▄▀ █▄█ ▀▀▀▀▀▀▀    
    ▀█▄█▀▄▀▄▄█▀▀▄▄██ ▀▄▀  ▀▄▄ ▄▄▀    
    ▄ ▀ ▀█▀▄ ▄██▄  ▀█▄▄▄▄███▄▄▀ ▄    
      ▄█▄▄▀█ ▀ ▀▀ ▄▄█▄▄ █▄█▄▄▄▄▄█    
    █▄▄ ▄▄▀ ▄▄▄█▀ ▀█  ▄██▀▄▄█ █ █    
    █ █▄▄▀▀▄▄ ▀▄▄▀▄█ ▄ ▄▀▀  ▄▀▄▄     
    ██▄▀▄▄▀█ █▄▀▄█ ▀█  ▀  ▄ ██▄ ▀    
    ▀▀▀ ▀ ▀ █ █ ▀ ██ ▀  █▀▀▀██▀▀     
    █▀▀▀▀▀█  █▄ ▀  ▀█▄█▀█ ▀ █▀       
    █ ███ █ ███ ▄▀███▄▀▀▀█▀▀▀▀ ▀▄    
    █ ▀▀▀ █ ▄▄█ ▄▄██▄▀█ ▀▄▄██▄█▀█    
    ▀▀▀▀▀▀▀ ▀ ▀▀▀  ▀   ▀▀▀           

```

----
### Telegram
Message [@LeviathanDevious](https://t.me/LeviathanDevious) or scan this QR code to see my profile:

```

    █▀▀▀▀▀█ ▄▀▀▄█ ▀█  █▀▀▀▀▀█    
    █ ███ █ ▀██▄█ █ ▀ █ ███ █    
    █ ▀▀▀ █ █▄ ▀▄█ █▀ █ ▀▀▀ █    
    ▀▀▀▀▀▀▀ █▄▀▄█ ▀ ▀ ▀▀▀▀▀▀▀    
    █▀ █▄▄▀█▄ █▀▄▀▀ ▄▄██▀ ▀▀▄    
    █   ▀█▀▄▀▀▀▄▄██▄█▀▄▀█  ▀▀    
    ▀█  ▄▀▀█ ▄▄▀█▀  ▀███ █▄▀█    
    ▀▄▀ ▄ ▀▀ ▀▄█ ▄▄▄█  ▀█ ▀▄▀    
    ▀▀▀▀  ▀ █▄▀██ ▄██▀▀▀██▀ ▄    
    █▀▀▀▀▀█ ▀ █ █▀  █ ▀ ██▄▀▀    
    █ ███ █ ▄  ▀▄▄▀ █▀▀███▄      
    █ ▀▀▀ █ ▄▄▄ ▄█▄██▄ ▀▀▄▀ ▀    
    ▀▀▀▀▀▀▀ ▀▀▀ ▀▀  ▀▀▀▀   ▀▀    

```


----
### Footnotes
[1]: To use *end-to-end encryption*, a 'secret chat' must be started instead of Telegram's default chat mode. Additionally, the secret sauce known as Telegram's cloud infrastructure makes it difficult to attest the platform's actual privacy, even with *e2e*. Use at your own discretion, or just DM me via Matrix instead.
";

pub const NOTFOUND: &'static str = "Not Found";
pub const NOTFOUND_DESCRIPTION_MD: &'static str = "Requested page could not be located. If you expected something in particular, please [open an issue](https://github.com/lilyanavalley/peace/issues) on this site's GitHub repo.";

pub const WIP: &'static str = "content coming soon";

pub const ACCESSIBILITY: &'static str = "Accessibility & Accomodations";
pub const ACCESSIBILITY_DESCRIPTION_MD: &'static str = "Adjust content appearances for enhanced readability, navigation and personal preference.

Should these accommodations not satisfy, kindly [open an issue](https://github.com/lilyanavalley/peace/issues) on this site's GitHub page.";

pub const FAVQUOTES: &'static str = "quote of the day";

pub const FOOTER: &'static str = "the home of";
pub const FOOTER_NAME: &'static str = "Lily Ana Valley";
pub const FOOTER_SOURCE: &'static str = "Source";
pub const FOOTER_ISSUES: &'static str = "Issues";
pub const FOOTER_LICENSE: &'static str = "License";
pub const FOOTER_STATUS: &'static str = "Status";
pub const FOOTER_CANARY: &'static str = "Canary";
