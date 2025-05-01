
use leptos::prelude::*;
use leptos_router::nested_router::Outlet;
use phosphor_leptos::*;
use crate::{ components, placeholders };


// #[component]
// pub fn Contact() -> impl IntoView {
//   view! {
//     <h1 class="text-center">Contact</h1>
//     <Outlet/>
//   }
// }

// #[component]
// pub fn ContactStart() -> impl IntoView {
//   view! {

//     <div class="flex flex-col grow self-center">

//       // ? Introduction to Contact Component.
//       <article class="w-3/4 self-center text-wrap">

//         <p>{ placeholders::CONTACT_DESCRIPTION }</p>

//         <components::article::Markdown markdown={ placeholders::CONTACT_INSTRUCTIONS_MD.to_string() }/>

//         <components::article::SectionIcon
//           icon=KEY
//         > 
//           <components::article::Markdown markdown={ placeholders::CONTACT_PGP_INSTRUCTIONS_MD.to_string() }/>
//           <components::code::Plain text={ placeholders::PGPKEY } />
//         </components::article::SectionIcon>
//       </article>

//       // ? 'Contact Me' form; starts a new correspondence.
//       <div id="messagecomposer" class="flex flex-col justify-center items-center self-center bg-[var(--color-deepblack)] w-3/4 sm:w-full" style="border: 4px solid #FFF; border-radius: 1rem; margin-top: 3rem;">
//         <form class="" action="/contact/start">
          
//           <div id="subjectline" class="flex items-center bg-[#333]" style="border-radius: 1rem;">
//             // Guest message subject.
//             <label for="subject" class="text-[#999]" style="margin-left: 1.5rem; margin-right: .5rem;">Subject</label>
//             <input type="text" id="subject" name="subject" placeholder="What’s the time?" class="grow b-1" style="border-radius: 1rem; padding: .5rem;" required></input><br/>
//           </div>

//           <div id="messagebody" class="flex flex-col" style="border-radius: 1rem;">
//             // Body of guest message.
//             <textarea id="messagebody" name="messagebody" placeholder="Message content" style="min-height: 8rem; border-color: #222;" required></textarea><br/>

//             // Send button.
//             <button onclick="submit" class="w-1/2 self-center text-[1.2rem]" style="border-radius: 1.5rem; margin: 1rem; margin-bottom: 0;">
//               <Icon icon=PAPER_PLANE_TILT/>
//               "Send"
//             </button>
//           </div>

//           // ? Warning bubble with 'respect me' text.
//           <div class="flex items-center text-[#AAA] p-[.5rem]">
//             <div style="margin: .25rem"><Icon icon=INFO size="2rem"/></div>
//             <p>{ placeholders::CONTACT_BE_NICE }</p>
//           </div>

//         </form>
//       </div>

//       // ? Information about 'Contact Me' security and transparency.
//       <article class="w-3/4 self-center text-wrap">

//         <components::article::Spacer/>

//         <components::article::SectionReveal
//           revealer_text={ placeholders::CONTACT_PRIVACY_REVEALER.to_string() }
//           icon=BINOCULARS
//         >

//           <components::article::SectionIcon
//             icon=CLOUD
//           >
//             <components::article::Markdown
//               markdown={ placeholders::CONTACT_PRIVACY_SECT_CLOUD_MD.to_string() }
//             />
//           </components::article::SectionIcon>

//           <components::article::SectionIcon
//             icon=DATABASE
//           >
//             <components::article::Markdown
//               markdown={ placeholders::CONTACT_PRIVACY_SECT_ANTISPAM_MD.to_string() }
//             />
//           </components::article::SectionIcon>
            
//           <components::article::SectionIcon
//             icon=BINOCULARS
//           >
//             <components::article::Markdown
//               markdown={ placeholders::CONTACT_PRIVACY_SECT_CLIENTSIDE_MD.to_string() }
//             />
//           </components::article::SectionIcon>

//         </components::article::SectionReveal>

//       </article>

//     </div>

//   }
// }

// #[component]
// pub fn ContactUpdate() -> impl IntoView {
//   let cid = "test123";
//   view! {
//     <div class="flex">
//       <p>Chat Code:</p>
//       <p><code>{ cid }</code></p>
//     </div>
//     <div id="chat" class="flex flex-col">
//       <div id="message-1" class="bg-[#444]">
//         "message bubble"
//       </div>
//     </div>
//     <div id="composer" class="flex">
//       <input _type="text"/>
//       <button>"Send"</button>
//     </div>
//   }
// }

// ? Temporary route while the contact route is being built.
#[component]
pub fn ContactAlternative() -> impl IntoView {
  view! {
    <div id="contact" class="flex flex-col w-3/4 self-center">
      <h1 class="text-center">"Contact"</h1>
      <components::article::Article>
        <components::article::Markdown
          markdown = { placeholders::CONTACT_ALTERNATIVE.to_string() }
        />
        <components::article::SectionReveal
          revealer_text = "My PGP Key".to_string()
          icon = KEY
        >
          "Key by fingerprint `0x37885CF7B9E39298` output:"
          <components::code::Plain text={ placeholders::PGPKEY }/>
        </components::article::SectionReveal>
      </components::article::Article>
    </div>
  }
}
