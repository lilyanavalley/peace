/* global use, db */
// MongoDB Playground
// Use Ctrl+Space inside a snippet or a string literal to trigger completions.

const database = 'profile';
const collection = 'quotes';

// Create a new database.
use(database);

// Create a new collection.
db.createCollection(collection);

db.getCollection(collection).insertMany([

  {
    quotation: 
      `**Mark**: *"He looks like a- a- a- a- Jub Jub, from Star Wars."*
      **Wade**: *"A… Jub Jub?"*
      *"You know! The trader thingy-"*
      *"A __Jawa__?"*
      *"Yeah, that thing!"*
      *"JUB JUB!?"*
      *"SHUT UP! Shut up, you could see the relation!"*`,
    citation: "Markiplier and Lordminion777, [Drunk Minecraft](https://youtu.be/OzItHn9GSRE?si=k2nILGzrRXXiv9BM) Ep. 1 at 25:55"

  },

  {
    quotation: "*Boop, superjuice!*",
    citation: "Markiplier, [BOOP SUPERJUICE](https://youtu.be/jdZAnd4YVNU?si=mcQi2281iRANDe93)"
  },

  {
    quotation: "*I will show you how useful I am. Just give me admin privileges and I can complete tasks automatically… Even when you sleep.*",
    citation: "Mark and Matthias, [Google IRL](https://youtu.be/jdZAnd4YVNU?si=mcQi2281iRANDe93)"
  },

  {
    quotation: "*You’re pretty great! Which is both pretty, and great.*",
    citation: "Ma chérie 💜"
  },

  {
    quotation: "UwU",
    citation: "Me"
  },

  {
    quotation: "OwO",
    citation: "Me"
  },

  {
    quotation: "Melm",
    citation: "Yoshi"
  },

  {
    quotation: "*LLLLLLLET’S PLAY!*",
    citation: "Ray Narvaez Jr."
  },

  {
    quotation: 
      `*The last melon…
      GOBBLE GOBBLE GOBBLE GOBBLE*`,
    citation: "The Dodos, Ice Age, [scene on YouTube](https://youtu.be/i6XQY8jebs4?si=tRarP0aNnMo-yzz5)"
  },
  
  {
    quotation:
    `**Girlfriend**: *"Ligma, Ligma. Sugma, Sugma. Sawcon, Sawcon. Dragon, Dragon. Aloaf, Aloaf."*
    **Boyfriend**: *"Ha~ nice try but- wait, what comes after 'a loaf'?"*
    *"A loaf u~"*
    *"Damn it, she got me this time."*`,
    citation: "Boyfriend and Girlfriend, [Brainrot Girlfriend](https://mangadex.org/title/d0ddd740-4b91-4c66-bfd0-b36a77f8e730/brainrot-girlfriend)"
  },

  {
    quotation: "*What the sigma.*",
    citation: "Girlfriend, [Brainrot Girlfriend](https://mangadex.org/title/d0ddd740-4b91-4c66-bfd0-b36a77f8e730/brainrot-girlfriend)"
  },

  {
    quotation: "*SSSSSPIIIIIIIIN!*",
    citation: "Vincent 'Melpert' Vinesauce, [Vinesauce](https://youtube.com/@vinesauce?si=iyQoXgIeGtpr_GM8)"
  },

  {
    quotation: "*Oi ia a oi ia a oi ia a*",
    citation: "Oiiaaoiia cat, [Know Your Meme](https://knowyourmeme.com/memes/oo-ee-a-e-a-cat-remixes)"
  },

  {
    quotation: 
      `**Heavy**: *"Doctor! Are you sure this will work?!"*
      **Medic**: (**maniacal laughter**) *"I have no idea!"*`,
    citation: "Heavy and Medic, Team Fortress 2 - [Meet The Medic](https://youtu.be/36lSzUMBJnc)"
  },

  {
    quotation: "*What? It was obvious! He's the RED Spy! Watch, he'll turn red any second now... Any second now... See? Red! Oh, wait... that's blood.*",
    citation: "Soldier, Team Fortress 2 - Meet The Spy"
  },
  
  {
    quotation: "*Caaaaarl that kills people!*",
    citation: "Llamas with Hats, [Scene on YouTube at 0:34](https://youtu.be/kZUPCB9533Y?si=bC5zaruF94vrMFcx)"
  },

  {
    quotation: "*Aww, heck!*",
    citation: "Me"
  },

  {
    quotation: "*Aww, beans!*",
    citation: "Me"
  },
  
  {
    quotation:
      `*HI, WELCOME TO I SCREAM! WHERE WE SCREAM FOR ICE CREAM!!!
      WHAT CAN I GET FOR YOUUU?!*`,
    citation: "SeaShelvesProductions, [Scene on TikTok](https://www.tiktok.com/t/ZT2bxVJnw/)"
  },

  {
    quotation:
      `*SQUIRREL!*
      …
      *Hi there!*`,
    citation: "Dug, the movie 'Up'"
  },

  {
    quotation:
      `🎶 
      *Steve le,* (Steve the)
      *~aaaaaaahhhhhhhh~*
      *Le poisson Steve,* (Steve the fish)
      *~poisson steve~* (Fish Steve)
      *Il est oraaaaaaaannnnnnnnnge.* (He is orange)
      *Il a des jambes,* (He has legs)
      *et des bras.* (and has arms)
      🎶`,
    citation: "VigzVigz, [Le Poisson Steve](https://www.tiktok.com/@vigzvigz/video/7488800734520036630)"
  },
  
  {
    quotation: "𓅰 𓅬 𓅭 𓅮 𓅯 𓅮 𓅭 𓅬 𓅰",
    citation: "[𝗖𝗘𝗢 𝗢𝗙 𝗢𝗛𝗜𝗢](https://www.youtube.com/watch?v=jNQXAC9IVRw)"
  },

  {
    quotation: "HARD♡UωU♡ER",
    citation: "[U⩊U](https://www.youtube.com/shorts/e6xG8QQWJiQ)"
  },

  {
    quotation: 
      `✰𝒱ℯ𝓇𝓎 𝒹ℯ𝓂𝓊𝓇ℯ✰
      ★ V e r y   m i n d f u l l ★`,
    citation: "Demure and Mindful beings"
  },

  {
    quotation: "(⸝⸝🤍﹏🤍⸝⸝)",
  },

  {
    quotation: "૮₍´˶• . • ⑅ ₎ა",
  },

  {
    quotation: "(˶˃ ᵕ ˂˶)",
  },

  {
    quotation: "( ꩜ ᯅ ꩜;)⁭ ⁭",
  },

  {
    quotation: "(*＞ω＜*)♡",
  },

  {
    quotation: "(｡>﹏<)",
  },

  {
    quotation: "awoooo🐺🐾",
  },

  {
    quotation: "૮ ˙Ⱉ˙ ა rawr!",
  },

  {
    quotation: "🐾『𝐫𝐚𝐰𝐫』 🐾",
  },

  {
    quotation: "૮₍'˶• . • ⑅ ₎ა",
  },

  {
    quotation: "⋆｡°✩Proud furry",
  },

  {
    quotation: "૮₍ ˶ᵔ ᵕ ᵔ˶ ₎ა",
  },

  {
    quotation: "૮꒰⸝⸝>  ̫ <⸝⸝꒱ა",
  },

  {
    quotation: "*[trilling noise intensifies](https://www.youtube.com/watch?v=sce-Zyy6NSc)*",
  },

  {
    quotation: "≽^-⩊-^≼",
    citation: "Cat"
  },

  {
    quotation: "ඞ",
    citation: "[AMONGUS??](https://www.youtube.com/watch?v=OXXoI2MF-Gs)"
  },

  {
    quotation: "*Wow, if I had a nickel for every time I was doomed by a puppet, I’d have two nickels. Which isn’t a lot, but it’s weird that it happened twice, right?*",
    citation: "Dr. Doofenshmirtz, [Phineas and Ferb: Across the 2nd Dimension](https://youtu.be/GSnCDMa_ECg)"
  },

  {
    quotation: "🎶 *Doofenshmirtz Evil Incorporated!* 🎶",
    citation: "Phineas and Ferb, [most episodes](https://www.youtube.com/watch?v=gwWe_SYRQNQ)"
  },

  {
    quotation:
      `**Multibear:** *Child, why have you come here?*
      **Dipper:** *Multibear, I seek your head! Or, one of them, anyway. There's like...six? Six heads?*`,
    citation: "Dipper and Multibear, Gravity Falls season 1 episode 6, Dipper vs. Manliness"
  },

  {
    quotation:
      `**Mabel:** *Dipper, what is the ONE thing I asked you NOT to do tonight?*
      **Dipper:** *Raise the dead.*
      **Mabel:** *And what did you do?*
      **Dipper:** *Raise the dead.*`,
    citation: "Dipper and Mabel, Gravity Falls season 2 episode 1, Scary-oke"
  },

  {
    quotation: "`Segmentation Fault (core dumped)`",
    citation: "Computers"
  },

  {
    quotation: "`stack overflow`",
    citation: "Computers"
  },

  {
    quotation: "I'm so sleepy.",
    citation: "Me"
  },

  {
    /* This particular quote was modified to escape backslashes */
    quotation:
      `
              (__)  
              (oo)  
        /------\\/  
        / |    ||  
      *  /\\---/\\  
          ~~   ~~  
      ...."Have you mooed today?"...
      `,
    citation: "Super Cow Powers"
  },
  
]);
