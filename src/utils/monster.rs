use super::consts::*;
use super::game_state::*;
use super::player::*;
use rand::Rng;

pub struct Monster {
    pub name: String,
    pub health_points: i32,
    pub base_damage: std::ops::RangeInclusive<i32>,
    pub level: usize,
    pub description: String,
    pub experience_given: i32,
    pub image: String,
}

impl Attack for Monster {
    fn get_attack_damage(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let roll_for_hit: i32 = rng.gen_range(self.base_damage.clone());
        roll_for_hit
    }
    fn receive_damage(&mut self, attack_damage: i32) {
        self.health_points -= attack_damage;
    }
    fn get_health_points(&self) -> i32 {
        self.health_points
    }
}

pub fn get_initial_monster() -> Monster {
    let mut rng = rand::thread_rng();
    let random_index: usize = rng.gen_range(0..MONSTERS.len());
    let selected_monster = MONSTERS[random_index];
    let (name, description, image) = selected_monster;
    let level = 1;

    Monster {
        name: name.to_string(),
        description: description.to_string(),
        image: image.to_string(),
        base_damage: 1..=MONSTER_BASE_RANGE_MAX_POINT * 2i32.pow(level as u32),
        experience_given: MONSTER_BASE_EXPERIENCE_GIVEN * 2i32.pow(level as u32),
        health_points: MONSTER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
        level: level as usize,
    }
}

pub fn get_random_monster(state: &mut GameState) -> Monster {
    let mut rng = rand::thread_rng();
    let random_index: usize = rng.gen_range(0..MONSTERS.len());
    let selected_monster = MONSTERS[random_index];
    let (name, description, image) = selected_monster;
    let level = state.player.level;

    Monster {
        name: name.to_string(),
        description: description.to_string(),
        image: image.to_string(),
        base_damage: 1..=MONSTER_BASE_RANGE_MAX_POINT + level as i32,
        experience_given: MONSTER_BASE_EXPERIENCE_GIVEN * 2i32.pow(level as u32),
        health_points: MONSTER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
        level: level as usize,
    }
}

const MONSTERS: [(&str, &str, &str); 15] = [
    (
        "Greta the Fierce",
        "A fearsome warrior from the northern realms, known for her unmatched strength and courage.",
        r#"
        w*W*W*W*w
         \"."."/
          //`\\
         (/a a\)
         (\_-_/) 
        .-~'='~-.
       /`~`"Y"`~`\
      / /(_ * _)\ \
     / /  )   (  \ \
     \ \_/\\_//\_/ / 
      \/_) '*' (_\/
        |       |
        |       |
        |       |
        |       |
        |       |
        |       |
        |       |
        |       |
        w*W*W*W*w     
        "#
    ),
    (
        "Thornback Terror",
        "A legendary creature with thorny spikes on its back, said to guard ancient treasures.",
        r#"
        , ,, ,                              
        | || |    ,/  _____  \.             
        \_||_/    ||_/     \_||             
          ||       \_| . . |_/              
          ||         |  L  |                
         ,||         |`==='|                
         |>|      ___`>  -<'___             
         |>|\    /             \            
         \>| \  /  ,    .    .  |           
          ||  \/  /| .  |  . |  |           
          ||\  ` / | ___|___ |  |     (     
       (( || `--'  | _______ |  |     ))  ( 
     (  )\|| (  )\ | - --- - | -| (  ( \  ))
     (\/  || ))/ ( | -- - -- |  | )) )  \(( 
      ( ()||((( ())|         |  |( (( () )
        "#
    ),
    (
        "Abyssal Spider",
        "A monstrous sea spider that lurks in the depths, leaving terror in its wake.",
        r#"
        ____                      ,
        /---.'.__             ____//
             '--.\           /.---'
        _______  \\         //
      /.------.\  \|      .'/  ______
     //  ___  \ \ ||/|\  //  _/_----.\__
    |/  /.-.\  \ \:|< >|// _/.'..\   '--'
       //   \'. | \'.|.'/ /_/ /  \\
      //     \ \_\/" ' ~\-'.-'    \\
     //       '-._| :H: |'-.__     \\
    //           (/'==='\)'-._\     ||
    ||                        \\    \|
    ||                         \\    '
    |/                          \\
                                 ||
                                 ||
                                 \\
        "#
    ),
    (
        "Blazing Phoenix",
        "A magnificent bird of flames, symbolizing rebirth and renewal.",
        r#"
        //.---.    .-'.
        ( (-/==^==.  /    ) ))
          /|))è é()./   .'
         ('-((\_/( ))..' /
          \ '-;_.-. ) ))
           '-(_ _)_\ ) )).'
            / ) (/_ ) \
        (( ( /\_/\,/|  ) ))
            /  .  '.'.' 
           (  .\  . '.___.
            \_| \  '.___/
             \'._;.___) 
              \_|-.\ |
               '--,-\'.
                  |/ \ )
                ._/   \|_
                       \ )
                        \|
                       ._)
        "#
    ),
    (
        "Shadowmaw Reaper",
        "A mysterious reaper of souls that dwells in the shadows, collecting lost spirits.",
        r#"
                                                   .""--.._
                                           []      `'--.._
                                           ||__           `'-,
                                         `)||_ ```'--..       \
                     _                    /|//}        ``--._  |
                  .'` `'.                /////}              `\/
                 /  .""".\              //{///    
                /  /_  _`\\            // `||
                | |(_)(_)||          _//   ||
                | |  /\  )|        _///\   ||
                | |L====J |       / |/ |   ||
               /  /'-..-' /    .'`  \  |   ||
              /   |  :: | |_.-`      |  \  ||
             /|   `\-::.| |          \   | ||
           /` `|   /    | |          |   / ||
         |`    \   |    / /          \  |  ||
        |       `\_|    |/      ,.__. \ |  ||
        /                     /`    `\ ||  ||
       |           .         /        \||  ||
       |                     |         |/  ||
       /         /           |         (   ||
      /          .           /          )  ||
     |            \          |             ||
    /             |          /             ||
   |\            /          |              ||
   \ `-._       |           /              ||
    \ ,//`\    /`           |              ||
     ///\  \  |             \              ||
    |||| ) |__/             |              ||
    |||| `.(                |              ||
    `\\` /`                 /              ||
       /`                   /              ||
      /                     |              ||
     |                      \              ||
    /                        |             ||
  /`                          \            ||
/`                            |            ||
`-.___,-.      .-.        ___,'            ||
         `---'`   `'----'`
        "#
    ),
    (
        "Frostbite Yeti",
        "A giant yeti covered in ice, dwelling in the frosty mountains of the north.",
        r#"
                         _,--~~~,
                       .'        `.
                       |           ;
                       |           :
                      /_,-==/     .'
                    /'`\*  ;      :      
                  :'    `-        :      
                  `~*,'     .     :      
                     :__.,._  `;  :      
                     `\'    )  '  `,     
                         \-/  '     )     
                         :'          \ _
                          `~---,-~    `,)
          ___                   \     /~`\
    \---__ `;~~~-------------~~~(| _-'    `,
  ---, ' \`-._____     _______.---'         \
 \--- `~~-`,      ~~~~~~                     `,
\----      )                                   \
\----.  __ /                                    `-
 \----'` -~____  
               ~~~~~--------,.___     
                                 ```\_
        "#
    ),
    (
        "Ironclad Goliath",
        "A colossal golem made of iron, a guardian of hidden treasures.",
        r#"
                         /[-])//  ___
                    __ --\ `_/~--|  / \ 
                  /_-/~~--~~ /~~~\\_\ /\
                  |  |___|===|_-- | \ \ \
_/~~~~~~~~|~~\,   ---|---\___/----|  \/\-\
~\________|__/   / // \__ |  ||  / | |   | |
         ,~-|~~~~~\--, | \|--|/~|||  |   | | 
         [3-|____---~~ _--'==;/ _,   |   |_|
                     /   /\__|_/  \  \__/--/ 
                    /---/_\  -___/ |  /,--| 
                    /  /\/~--|   | |  \///
                   /  / |-__ \    |/
                  |--/ /      |-- | \  
                 \^~~\\/\      \   \/- _
                  \    |  \     |~~\~~| \ 
                   \    \  \     \   \  | \
                     \    \ |     \   \    \
                      |~~|\/\|     \   \   |
                     |   |/         \_--_- |\
                     |  /            /   |/\/
                      ~~             /  /   
                                    |__/           
        "#
    ),
    (
        "Wispwood Enchanter",
        "An enchanting spirit of the wispwood forest, protector of its magical secrets.",
        r#"
        .-----.
        \ ' /   _/    )/
       - ( ) -('---''--)
        / . \((()\^_^/)()
         \\_\ (()_)-((()()
          '- \ )/\._./(()
            '/\/( X   ) \
            (___)|___/ ) \
                 |.#_|(___)
                /\    \ ( (_
                \/\/\/\) \\
                | / \ |
                |(   \|
               _|_)__|_\_
               )...()...(
                | (   \ |     
             .-'__,)  (  \
                       '\_-,
        "#
    ),
    (
        "Thunderhoof Minotaur",
        "A Minotaur with thunderous hooves, challenging all who enter its labyrinth.",
        r#"
        .      .
        |\____/|
       (\|----|/)
        \ 0  0 /
         |    |
      ___/\../\____
     /     --       \
    /  \         /   \
   |    \___/___/(   |
   \   /|  }{   | \  )
    \  ||__}{__|  |  |
     \  |;;;;;;;\  \ / \_______
      \ /;;;;;;;;| [,,[|======'
        |;;;;;;/ |     /
        ||;;|\   |
        ||;;/|   /
        \_|:||__|
         \ ;||  /
         |= || =|
         |= /\ =|
         /_/  \_\
        "#
    ),
    (
        "Duskwing Harpy",
        "A harpy with wings as dark as the night, a skilled hunter in the moonlight.",
        r#"
        ,                                      ,
        |\                                      /|
     ,   \'._ ,                           ,  _.'/   ,
     |\  {'. '-`\,      ,-._**_.-,      ,/`-' .'}  /|
      \`'-'-.  '.`\      \*____*/      /`.'  .-'-'`/
    ,'-'-._  '.  ) )     /`    `\     ( (  .'  _.-'-',
    |\'- _ '.   , /     /  /""\  \     \ ,  .'  _ -'/|
     \'-.   .  ; (      \_|^  ^|_/      ) ;   .  .-'/
      `'--, . ;  {`-.      \__/      .-'}  ; . ,--'`
      '--`_. ;  { ^  \    _|  |_    /  ^ }  ; ._`--'
      `,_.--  ;  { ^  `-'`      `'-`  ^ }  ;  --._,`
        ,_.-    ; {^    /        \    ^} ;    -._, 
         ,_.-`), /\{^,/\\_'_/\_'_//\,^}/\ ,(`-._,
           _.'.-` /.'   \        /   `.\ `-.'._
          `  _.' `       \      /       ` '._   `
                        .-'.  .'-.
                      .'    `` ^  '.
                     /  ^ ^   ^  ^  \
                     | ^    ^   ^   |
                    /^   ^/    \  ^  \
                    \,_^_/    ^ \_,^./
                     /=/  \^   /  \=\
                 __ /=/_   | ^|   _\=\ __
               <(=,'=(==,) |  | (,==)=',=)>
                 /_/|_\    /  \    /_|\_\
                 `V (=|  .'    '.  |=) V`
                     V  / _/  \_ \  V
                       `"` \  / `"`
                            \(
        "#
    ),
    (
        "Venomspine Basilisk",
        "A basilisk with venomous spines, said to turn its prey into stone with a single glance.",
        r#"                             
        ___,---.__          /'|`\          __,---,___          
     ,-'    \`    `-.____,-'  |  `-.____,-'    //    `-.       
   ,'        |           ~'\     /`~           |        `.      
  /      ___//              `. ,'          ,  , \___      \    
 |    ,-'   `-.__   _         |        ,    __,-'   `-.    |    
 |   /          /\_  `   .    |    ,      _/\          \   |   
 \  |           \ \`-.___ \   |   / ___,-'/ /           |  /  
  \  \           | `._   `\\  |  //'   _,' |           /  /      
   `-.\         /'  _ `---'' , . ``---' _  `\         /,-'     
      ``       /     \    ,='/ \`=.    /     \       ''          
              |__   /|\_,--.,-.--,--._/|\   __|                  
              /  `./  \\`\ |  |  | /,//' \,'  \                  
             /   /     ||--+--|--+-/-|     \   \                 
            |   |     /'\_\_\ | /_/_/`\     |   |                
             \   \__, \_     `~'     _/ .__/   /            
              `-._,-'   `-._______,-'   `-._,-'
        "#
    ),
    (
        "Stardust Dragon",
        "A celestial dragon from the starry skies, guarding ancient constellations.",
        r#"
                 ___====-_  _-====___
           _--^^^#####//      \\#####^^^--_
        _-^##########// (    ) \\##########^-_
       -############//  |\^^/|  \\############-
     _/############//   (@::@)   \\############\_
    /#############((     \\//     ))#############\
   -###############\\    (oo)    //###############-
  -#################\\  / VV \  //#################-
 -###################\\/      \//###################-
_#/|##########/\######(   /\   )######/\##########|\#_
|/ |#/\#/\#/\/  \#/\##\  |  |  /##/\#/  \/\#/\#/\#| \|
`  |/  V  V  `   V  \#\| |  | |/#/  V   '  V  V  \|  '
   `   `  `      `   / | |  | | \   '      '  '   '
                    (  | |  | |  )
                   __\ | |  | | /__
                  (vvv(VVV)(VVV)vvv)
        "#
    ),
    (
        "Molten Core Elemental",
        "An elemental creature born of molten lava, capable of burning everything in its path.",
        r#"
                                                     ,--,  ,.-.
               ,                   \,       '-,-`,'-.' | ._
              /|           \    ,   |\         }  )/  / `-,',
              [ ,          |\  /|   | |        /  \|  |/`  ,`
              | |       ,.`  `,` `, | |  _,...(   (      .',
              \  \  __ ,-` `  ,  , `/ |,'      Y     (   /_L\
               \  \_\,``,   ` , ,  /  |         )         _,/
                \  '  `  ,_ _`_,-,<._.<        /         /
                 ', `>.,`  `  `   ,., |_      |         /
                   \/`  `,   `   ,`  | /__,.-`    _,   `\
               -,-..\  _  \  `  /  ,  / `._) _,-\`       \
                \_,,.) /\    ` /  / ) (-,, ``    ,        |
               ,` )  | \_\       '-`  |  `(               \
              /  /```(   , --, ,' \   |`<`    ,            |
             /  /_,--`\   <\  V /> ,` )<_/)  | \      _____)
       ,-, ,`   `   (_,\ \    |   /) / __/  /   `----`
      (-, \           ) \ ('_.-._)/ /,`    /
      | /  `          `/ \\ V   V, /`     /
   ,--\(        ,     <_/`\\     ||      /
  (   ,``-     \/|         \-A.A-`|     /
 ,>,_ )_,..(    )\          -,,_-`  _--`
(_ \|`   _,/_  /  \_            ,--`
 \( `   <.,../`     `-.._   _,-`
        "#
    ),
    (
        "Moonshadow Assassin",
        "An elusive assassin who moves like a shadow in the moonlit night.",
        r#"
        -. -. `.  / .-' _.'  _
        .--`. `. `| / __.-- _' `
       '.-.  \  \ |  /   _.' `_
       .-. \  `  || |  .' _.-' `.
     .' _ \ '  -    -'  - ` _.-.
      .' `. %%%%%   | %%%%% _.-.`-
    .' .-. ><(@)> ) ( <(@)>< .-.`.
      (("`(   -   | |   -   )'"))
     / \\#)\    (.(_).)    /(#//\
    ' / ) ((  /   | |   \  )) (`.`.
    .'  (.) \ .md88o88bm. / (.) \)
      / /| / \ `Y88888Y' / \ | \ \
    .' / O  / `.   -   .' \  O \ \\
     / /(O)/ /| `.___.' | \\(O) \
      / / / / |  |   |  |\  \  \ \
      / / // /|  |   |  |  \  \ \  
    _.--/--/'( ) ) ( ) ) )`\-\-\-._
   ( ( ( ) ( ) ) ( ) ) ( ) ) ) ( ) )
        "#
    ),
    (
        "Crystalwing Pegasus",
        "A graceful Pegasus with crystal wings, a symbol of purity and grace.",
        r#"
        \ __
        --==/////////////[})))==*
                         / \ '          ,|
                            `\`\      //|                             ,|
                              \ `\  //,/'                           -~ |
           )             _-~~~\  |/ / |'|                       _-~  / ,
          ((            /' )   | \ / /'/                    _-~   _/_-~|
         (((            ;  /`  ' )/ /''                 _ -~     _-~ ,/'
         ) ))           `~~\   `\\/'/|'           __--~~__--\ _-~  _/, 
        ((( ))            / ~~    \ /~      __--~~  --~~  __/~  _-~ /
         ((\~\           |    )   | '      /        __--~~  \-~~ _-~
            `\(\    __--(   _/    |'\     /     --~~   __--~' _-~ ~|
             (  ((~~   __-~        \~\   /     ___---~~  ~~\~~__--~ 
              ~~\~~~~~~   `\-~      \~\ /           __--~~~'~~/
                           ;\ __.-~  ~-/      ~~~~~__\__---~~ _..--._
                           ;;;;;;;;'  /      ---~~~/_.-----.-~  _.._ ~\     
                          ;;;;;;;'   /      ----~~/         `\,~    `\ \        
                          ;;;;'     (      ---~~/         `:::|       `\\.      
                          |'  _      `----~~~~'      /      `:|        ()))),      
                    ______/\/~    |                 /        /         (((((())  
                  /~;;.____/;;'  /          ___.---(   `;;;/             )))'`))
                 / //  _;______;'------~~~~~    |;;/\    /                ((   ( 
                //  \ \                        /  |  \;;,\                 `   
               (<_    \ \                    /',/-----'  _> 
                \_|     \\_                 //~;~~~~~~~~~ 
                         \_|               (,~~   
                                            \~\
                                             ~~
        "#
    ),
];
