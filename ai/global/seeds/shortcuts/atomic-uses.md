# What things are for and what creatures can do — from ATOMIC-2020 (generated, 2026-10-01)

**Gist.** Common sense about things and creatures from our corpus: `use_<action>` — things usually used to do it (ObjectUse: knife, axe, saw → cut; apple, leaf → fall; sword, rifle → kill); `can_<action>` — who can do it (CapableOf: bird → fly, cat → eat). Queries: `global::uses(word)`, `global::capable(word)`.

**Conditions and exceptions.** The first word of the ATOMIC answer is taken as the action (crude: "butter bread" → butter); frequency ≥ 2; up to 6 actions per thing. A reference (`category`), not events — starts no consequence chains.

**Sources.** ATOMIC-2020 (CC BY 4.0); rebuild: `duckdb < global/data/atomic-uses.sql`, then this file from `data/runs/atomic/uses.tsv`.

```category
can_drink: horse human
can_eat: cat fish
can_fence: farmer rancher
can_find: pig searcher
can_fly: bird mosquito
can_govern: king ruler
can_grow: grass plant
can_hear: animal dog
can_hunt: cat hunter lion
can_jump: frog grasshopper horse kangaroo
can_kill: cat hunter
can_live: animal fish
can_run: dog horse
can_school: fish teacher
can_spot: hawk hunter
can_strike: human pitcher
can_travel: astronaut bird
can_walk: cat model
use_add: banana batter brain calculator ketchup lemon milk olive onion paintbrush pepperoni
use_age: cheese wine
use_apply: lotion oil paintbrush roller
use_arrive: car train
use_attach: bow clamp harness nail plug
use_attack: sword weapon
use_bake: flour pan
use_block: hat lead shield
use_blow: bomb handkerchief whistle
use_boil: broth kettle pasta saucepan
use_break: crowbar hammer mallet pickaxe vase
use_bring: backpack beer cake dessert suitcase
use_build: metal steel tree wood
use_burn: lighter match tobacco wood
use_buy: dollar supermarket
use_buying: dollar supermarket
use_call: cell whistle
use_carry: airplane ambulance backpack bag baggage bottle box briefcase bucket bus cage carriage cart container crate handbag platter pocket satchel shirt sled suitcase tray truck wagon wallet wheelbarrow
use_carrying: artery bag baggage briefcase bucket can handbag satchel wagon
use_carve: awl dagger
use_catch: bait fish glove lighter net rod
use_celebrate: beer cake champagne
use_check: calculator clock mirror scale
use_chew: mouth tooth
use_chop: axe knife
use_clean: broom cola dumpster lemon mop sink sponge sweeper toothpick towel vinegar
use_climb: ladder tower
use_contain: backpack bottle bowl box jug pan suitcase tray wallet
use_control: button leash mouse whip
use_cook: bacon pan saucepan steak toaster
use_cool: aloe drink
use_cover: clothing hat jacket mask robe scarf shirt tunic wig
use_covering: shirt skin
use_crack: saxophone trombone
use_create: batter cell firewood lighter printer string
use_cut: axe dagger diamond drill hair knife saw
use_cutting: diamond saw
use_decorate: anchor bow earring icing jacket ribbon saddle
use_deliver: hospital pulpit
use_dig: shovel spade
use_display: bookcase iphone shelf vase
use_donate: blood clothing junk
use_draw: pencil ruler
use_dress: clothing costume suit uniform
use_drill: diamond drill
use_drink: beer bottle champagne coffee cup juice liquid mug wine
use_drinking: cup mug
use_drive: ambulance bus car driver hammer motorcycle taxi truck vehicle
use_dry: handkerchief towel
use_eat: bacon banana bean bowl broccoli cake candy cereal cutlery dish food fork fruit pie plate popcorn salad sandwich soup spoon table tablespoon toast utensil
use_eating: dish meat mouth spoon
use_enter: apartment keyboard
use_fall: apple leaf skateboard
use_fasten: screw stapler
use_feed: bird bottle bread carrot crop fly food grain meatball milk muffin popcorn seed soup steak toast
use_fill: bowl candy furniture juice liquid milk oatmeal pen pencil pump tank
use_fire: bullet cannon pistol
use_fly: airplane jet rocket
use_follow: diet uniform
use_fry: bacon oil pan
use_getting: apple artery bell bus heart skin toothpick
use_grab: apple fork glove utensil
use_grinding: mill tooth
use_grow: hair plant seed tree
use_growing: bean wheat
use_hang: closet nail screw strap wardrobe
use_hear: ear home speaker
use_heat: kettle saucepan toaster
use_help: calculator coffee drink lawyer trainer
use_hide: barrel briefcase cloak closet coat container disguise glove hoodie mask mattress pocket tree trunk wardrobe
use_hit: arrow ball fist hammer hand net rod
use_hold: arm backpack bag baggage belt bookcase bottle bowl box bread briefcase bucket building cage can cart carton clasp clip container cooler crate cup desk dish dumbbell finger glove hand handbag harness hotel jar jug leash manger match matchbox mug nail neck net pan pistol pitcher plate platter pocket quiver ribbon saucer screw shelf stadium strap studio suitcase table tank teapot toothpick tray tub urn vase wallet wardrobe
use_holding: arm bag baggage bucket can chapel clip container cup drawer head jar mug plate screw tub wallet
use_house: barn garage home
use_hydrate: drink liquid
use_ignite: lighter match
use_keeping: bookcase clock closet coat cupboard file fur garage shoe umbrella
use_kick: ball boot foot shoe
use_kicking: foot leg
use_kill: bomb cannon knife rifle sword weapon
use_killing: bullet cannon pistol rifle sword
use_lay: bed bench mattress towel
use_lie: bed mattress
use_lift: arm hoist muscle
use_lifting: arm hoist
use_light: bulb lighter match
use_listen: ear headphone
use_live: apartment bungalow farmhouse furniture home house
use_living: bungalow home planet
use_load: bullet trunk
use_look: closet clothing costume disguise dress mirror suit tower vest
use_making: apple barn bean bell bread bucket cornet cymbal feather fruit fur gong king leaf metal mill pulpit saxophone steel tooth wheat
use_mark: cake highlighter pen pencil
use_measure: beaker ruler scale tablespoon teaspoon
use_measuring: cup foot ruler teaspoon
use_mix: beaker bowl cereal cream flour grass juice ketchup milk spoon tumbler whisk
use_move: cart fork furniture hoe mouse shovel spoon truck wheelbarrow
use_navigate: compass mouse
use_near: ship train
use_offer: beer sandwich
use_open: corkscrew crowbar cutlery key shutter
use_pack: box suitcase
use_park: car garage
use_part: arm leg
use_pay: cheque wallet
use_perform: preacher skateboard studio
use_pick: car fork hand highlighter puppy shovel taxi
use_pin: button pin
use_place: altar bait bow bowl box cart cell coat cooler desk dish furniture garage plate pocket seashell shelf table tomato tray trunk vase
use_play: ball bow bugle clarinet drum dvd fiddle flute glove guitar horse joystick keyboard leg organ piano poodle rook saxophone speaker stadium table toy trombone trumpet uniform violin
use_playing: child clarinet cymbal fiddle guitar king organ trumpet violin
use_point: compass finger
use_pour: bottle carton gravy jug liquid milk mug pan pitcher soup syrup tray
use_practice: ball guitar keyboard mirror piano skateboard studio
use_pray: altar chapel shrine
use_praying: chapel temple
use_preach: preacher pulpit
use_present: ring suit
use_press: button finger
use_prevent: apron diet flour harness leash mask robe
use_protect: apron brace cap coat dog glove hat headphone helmet jacket lotion mask orphanage shield shoe skin turban umbrella
use_protecting: hair shoe skin
use_protection: house rifle umbrella
use_provide: food tree
use_pull: hat horse pistol plug pocket shelf shirt string tractor wagon
use_push: button cart hand helmet
use_putting: cupboard saucer
use_reach: ladder stool
use_record: pen studio
use_relaxing: armchair couch
use_religious: altar temple
use_remove: comb corkscrew mop shovel sponge toothpick vinegar wrench
use_resting: armchair footstool
use_ride: ambulance bicycle bus helmet horse motorcycle saddle skateboard taxi tractor vehicle
use_riding: horse saddle
use_rinse: liquid sink
use_roll: barbell barrel roller
use_rub: lotion oil
use_run: bicycle gym ship tractor treadmill
use_sail: boat ship
use_scare: puppet trombone weapon whistle worm
use_scoop: fork ladle shovel spoon
use_scrub: mop sponge
use_secure: leash strap
use_see: clock eye mirror scale telescope tower
use_sell: building house lemonade
use_serve: beer biscuit bread cake dessert dish food plate platter wine
use_set: alarm building clock plate stool table tray
use_shade: hat tree umbrella
use_shape: chisel file
use_shoot: arrow bullet cannon crossbow pistol rifle
use_shooting: bullet cannon pistol rifle
use_show: medal mirror necklace ring shield uniform
use_signal: alarm whistle
use_singing: guitar mouth
use_sip: mug wine
use_sit: bed bench desk drum furniture pew saddle stool table throne vehicle
use_sitting: armchair chair pew saddle
use_sleep: bed hotel mattress
use_slice: cutter knife saw
use_smoke: pipe tobacco
use_soak: bathtub tub
use_spray: champagne mace
use_spread: butter knife sponge
use_stab: dagger knife toothpick
use_stack: firewood iphone
use_stand: bathtub chair footstool spot
use_start: button key lighter match rook
use_stay: apartment coat coffee helmet hotel jacket umbrella
use_stir: carrot ladle spoon teaspoon whisk
use_stirring: tablespoon teaspoon
use_stop: brake clog
use_storage: jar trunk
use_store: backpack barrel bookcase box briefcase cage can cart carton chest closet container cooler crate cupboard drawer file garage jar loft matchbox pocket safe shelf suitcase tank trunk tub wallet
use_storing: barn briefcase can chest closet cupboard drawer file garage jar shelf trunk
use_study: desk planet
use_sweep: broom sweeper
use_symbolize: crown ring
use_taking: artery baggage rook tub
use_teach: building guitar
use_test: brain finger hair
use_think: brain head
use_throw: ball banana biscuit cap dumpster helmet junk mansion orange pie rice
use_tie: charger ribbon scarf string
use_time: clock watch
use_toast: champagne toaster
use_transport: airplane ambulance bus cage car carriage crate hose taxi truck vehicle wagon
use_transporting: artery wagon
use_travel: airplane baggage bicycle boat bus car taxi truck vehicle
use_turn: button hand key wrench
use_unlock: apartment key safe
use_wake: alarm bugle coffee
use_walk: foot house leash
use_wash: bathtub coffee drink juice liquid milk mop sink sponge
use_watch: dvd mirror stadium telescope
use_wear: apron backpack beanie blouse cap clothing coat costume dress earring frock halter hat helmet jacket mask medal necklace nightgown ring robe scarf shirt slip spike suit sweater thong top uniform wrist
use_weigh: anchor scale
use_wet: hose sink
use_wipe: finger handkerchief mop sponge towel
use_work: desk gym
use_worship: altar chapel temple
use_wrap: bow scarf towel
use_write: cheque desk keyboard paintbrush parchment pen pencil slip
```
