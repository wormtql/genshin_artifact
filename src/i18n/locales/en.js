export default {
    webName: "MonaUranai",
    intro: {
        opensource: "Open source",
        opensourceMonaDescription: "- Frontend<br>- Mona DSL<br>- DSL Book",
        opensourceYasDescription: "- Yas scanner",

        useCase1: "Reactive damage calculator/Single Character Artifact Optimization",
        useCase1Description: "- Using advanced algorithm to optimize artifacts, to get maximum damage and more<br>- reactive calculator",
        useCase2: "Team artifacts optimization",
        useCase2Description: "Optimize a team of characters without artifacts conflict",
        useCase3: "Artifact potential",
        useCase3Description: "Potential is the expected score an artifact can achieve when upgraded to lvl20, which can be used to filter good artifacts",
        useCase4: "Artifact scan",
        useCase4Description: "Export your artifacts easily",

        feedback: "Feedback",
        fbGithubIssue: "Github Issue",
        fbIssueDescription: "Create an issue on github",
        // fbQQ: "QQ group",
        // fbQQDescription: "Give feed backs （群号：801106595）",
        // fbNGA: "NGA讨论帖",
        // fbNGADescription: "在NGA进行反馈"
    },
    nav: {
        nav: "Navigation",
        home: "Home",
        repo: "My Repo",
        account: "Account",
        artifact: "Artifact",
        kumi: "Artifact Group",
        preset: "Preset",
        compute: "Computation",
        calculate: "Calculator",
        teamOptimize: "Team Optimize",
        potential: "Artifact Potential",
        monaDB: "Mona DB",
        other: "Other",
        playground: "Playground",
        about: "About",
        help: "Help",
        exportTool: "Export Tools",
        link: "Other Links",
        setup: "Settings",
        bestSet: "Best Set",
    },
    misc: {
        character: "Character",
        mainStat: "Main Stat",
        subStat: "Sub Stat",
        any: "any",
        copy: "Copy",
        "import": "Import",
        "export": "Export",
        cancel: "Cancel",
        confirm: "Confirm",
        artifact: "Artifact",
        clear: "Clear",
        recommend: "Recommend",
        total: "Total",
        scan: "Scan",
        artifactSet: "Artifact set",
        hint: "Hint",
        lock: "Lock",
        unlock: "Unlock",
        del: "Delete",
        edit: "Edit",
        rename: "Rename",
        search: "Search",
        calculator: "Calculator",
        preset: "Preset",
        artSet: "Artifact Set",
        artSlot: "Artifact Slot",
        quality: "Quality",
        lvl: "Lvl",
        random: "Random",
        cont: "Continue",
        flower: "Flower of Life",
        Flower: "Flower of Life",
        feather: "Plume of Death",
        Feather: "Plume of Death",
        sand: "Sands of Eon",
        Sand: "Sands of Eon",
        cup: "Goblet of Eonothem",
        Goblet: "Goblet of Eonothem",
        head: "circlet of Logos",
        Head: "circlet of Logos",
        algo: "Algorithm",
        skill: "Skill",
        conste: "Constellation",
        weapon: "Weapon",
        refine: "Refine",
        tf: "Target Function",
        code: "Code",
        charSpecific: "Character Specific",
        general: "General",
        dmg: "DMG",
        type1: "Type",
        art4: "Set4: ",
        stat: "Stat",
        value: "Value",
        stat1: "Valuable Stat", // todo
        rollCount: "Roll Count",
        selected: "Selected",
        panel: "Stats",
        lang: "Language",
    },
    stat: {
        cureEffect: "Healing Bonus",
        criticalDamage: "Crit DMG",
        critical: "Crit Rate",
        attackStatic: "ATK",
        attackPercentage: "ATK%",
        elementalMastery: "Elemental Mastery",
        recharge: "Energy Recharge",
        lifePercentage: "HP%",
        defendPercentage: "DEF%",
        lifeStatic: "HP",
        defendStatic: "DEF",
        thunderBonus: "Electro DMG Bonus",
        fireBonus: "Pyro DMG Bonus",
        waterBonus: "Hydro DMG Bonus",
        iceBonus: "Cryo DMG Bonus",
        windBonus: "Anemo DMG Bonus",
        rockBonus: "Geo DMG Bonus",
        dendroBonus: "Dendro DMG Bonus",
        physicalBonus: "Physical DMG Bonus",
        shield: "Shield Strength",
        base: "Base Stats",
        advanced: "Advanced Stats",
        ele: "Elemental Type",
    },
    artifact: {
        "DesertPavilionChronicle": {
            "setName": "Desert Pavilion Chronicle",
            "items": [
                "The First Days of the City of Kings",
                "End of the Golden Realm",
                "Timepiece of the Lost Path",
                "Defender of the Enchanting Dream",
                "Legacy of the Desert High-Born",
            ],
            "effects": {
                "2": "Anemo DMG Bonus +15%",
                "4": "After Charged Attacks hit opponents, this character's Normal Attack SPD will increase by 10% while Normal, Charged, and Plunging Attack DMG will increase by 30% for 10s.",
            }
        },
        "FlowerOfParadiseLost": {
            "setName": "Flower of Paradise Lost",
            "items": [
                "Moon Maiden's Myriad",
                "Wilting Feast",
                "A Moment Congealed",
                "Secret-Keeper's Magic Bottle",
                "Amethyst Crown",
            ],
            "effects": {
                "2": "Elemental Mastery +80",
                "4": "The equipping character's Bloom, Hyperbloom, and Burgeon reaction DMG are increased by 50%. Additionally, when the equipping character triggers Bloom, Hyperbloom, or Burgeon they will gain another 25% bonus to the effect mentioned prior. Each stack of this lasts 10s. Max 4 stacks simultaneously. This effect can only be triggered once per second. The character who equips this can still trigger its effects when not on the field.",
            }
        },
        "DeepwoodMemories": {
            "setName": "Deepwood Memories",
            "items": [
                "Labyrinth Wayfarer",
                "Scholar of Vines",
                "A Time of Insight",
                "Lamp of the Lost",
                "Laurel Coronet",
            ],
            "effects": {
                "2": "Dendro DMG Bonus +15%.",
                "4": "After Elemental Skills or Bursts hit opponents, the targets’ Dendro RES will be decreased by 30% for 8s. This effect can be triggered even if the equipping character is not on the field.",
            }
        },
        "GildedDreams": {
            "setName": "Gilded Dreams",
            "items": [
                "Dreaming Steelbloom",
                "Feather of Judgment",
                "The Sunken Years",
                "Honeyed Final Feast",
                "Shadow of the Sand King",
            ],
            "effects": {
                "2": "Elemental Mastery +80.",
                "4": "Within 8s of triggering an Elemental Reaction, the character equipping this will obtain buffs based on the Elemental Type of the other party members. ATK is increased by 14% for each party member whose Elemental Type is the same as the equipping character, and Elemental Mastery is increased by 50 for every party member with a different Elemental Type. Each of the aforementioned buffs will count up to 3 characters. This effect can be triggered once every 8s. The character who equips this can still trigger its effects when not on the field.",
            }
        },
        "VermillionHereafter": {
            "setName": "Vermillion Hereafter",
            "items": [
                "Flowering Life",
                "Feather of Nascent Light",
                "Solar Relic",
                "Moment of the Pact",
                "Thundering Poise"
            ],
            "effects": {
                "2": "ATK +18%.",
                "4": "After using an Elemental Burst, this character will gain the Nascent Light effect, increasing their ATK by 8% for 16s. When the character's HP decreases, their ATK will further increase by 10%. This further increase can occur this way a maximum of 4 times. This effect can be triggered once every 0.8s. Nascent Light will be dispelled when the character leaves the field. If an Elemental Burst is used again during the duration of Nascent Light, the original Nascent Light will be dispelled."
            }
        },
        "bloodstainedChivalry": {
            "setName": "Bloodstained Chivalry",
            "items": [
                "Bloodstained Flower of Iron",
                "Bloodstained Black Plume",
                "Bloodstained Final Hour",
                "Bloodstained Chevalier's Goblet",
                "Bloodstained Iron Mask"
            ],
            "effects": {
                "4": "After defeating an opponent, increases Charged Attack DMG by 50%, and reduces its Stamina cost to 0 for 10s.",
                "2": "Physical DMG +25%"
            }
        },
        "exile": {
            "setName": "The Exile",
            "items": [
                "Exile's Flower",
                "Exile's Feather",
                "Exile's Pocket Watch",
                "Exile's Goblet",
                "Exile's Circlet"
            ],
            "effects": {
                "4": "Using an Elemental Burst regenerates 2 Energy for all party members (excluding the wearer) every 2s for 6s. This effect cannot stack.",
                "2": "Energy Recharge +20%"
            }
        },
        "EchoesOfAnOffering": {
            "setName": "Echoes of an Offering",
            "items": [
                "Soulscent Bloom",
                "Jade Leaf",
                "Symbol of Felicitation",
                "Chalice of the Font",
                "Flowing Rings"
            ],
            "effects": {
                "2": "ATK +18%.",
                "4": "When Normal Attacks hit opponents, there is a 36% chance that it will trigger Valley Rite, which will increase Normal Attack DMG by 70% of ATK. This effect will be dispelled 0.05s after a Normal Attack deals DMG. If a Normal Attack fails to trigger Valley Rite, the odds of it triggering the next time will increase by 20%. This trigger can occur once every 0.2s."
            }
        },
        "viridescentVenerer": {
            "setName": "Viridescent Venerer",
            "items": [
                "In Remembrance of Viridescent Fields",
                "Viridescent Arrow Feather",
                "Viridescent Venerer's Determination",
                "Viridescent Venerer's Vessel",
                "Viridescent Venerer's Diadem"
            ],
            "effects": {
                "2": "Anemo DMG Bonus +15%",
                "4": "Increases Swirl DMG by 60%. Decreases opponent's Elemental RES to the element infused in the Swirl by 40% for 10s."
            }
        },
        "thunderSmoother": {
            "setName": "Thundersoother",
            "items": [
                "Thundersoother's Heart",
                "Thundersoother's Plume",
                "Hour of Soothing Thunder",
                "Thundersoother's Goblet",
                "Thundersoother's Diadem"
            ],
            "effects": {
                "4": "Increases DMG against opponents affected by Electro by 35%.",
                "2": "Electro RES increased by 40%."
            }
        },
        "maidenBeloved": {
            "setName": "Maiden Beloved",
            "items": [
                "Maiden's Distant Love",
                "Maiden's Heart-stricken Infatuation",
                "Maiden's Passing Youth",
                "Maiden's Fleeting Leisure",
                "Maiden's Fading Beauty"
            ],
            "effects": {
                "2": "Character Healing Effectiveness +15%",
                "4": "Using an Elemental Skill or Burst increases healing received by all party members by 20% for 10s."
            }
        },
        "wandererTroupe": {
            "setName": "Wanderer's Troupe",
            "items": [
                "Troupe's Dawnlight",
                "Bard's Arrow Feather",
                "Concert's Final Hour",
                "Wanderer's String-Kettle",
                "Conductor's Top Hat"
            ],
            "effects": {
                "2": "Increases Elemental Mastery by 80.",
                "4": "Increases Charged Attack DMG by 35% if the character uses a Catalyst or a Bow."
            }
        },
        "thunderingFury": {
            "setName": "Thundering Fury",
            "items": [
                "Thunderbird's Mercy",
                "Survivor of Catastrophe",
                "Hourglass of Thunder",
                "Omen of Thunderstorm",
                "Thunder Summoner's Crown"
            ],
            "effects": {
                "2": "Electro DMG Bonus +15%",
                "4": "Increases damage caused by Overloaded, Electro-Charged and Superconduct by 40%. Triggering such effects decreases Elemental Skill CD by 1s. Can only occur once every 0.8s."
            }
        },
        "emblemOfSeveredFate": {
            "setName": "Emblem of Severed Fate",
            "items": [
                "Magnificent Tsuba",
                "Sundered Feather",
                "Storm Cage",
                "Scarlet Vessel",
                "Ornate Kabuto"
            ],
            "effects": {
                "2": "Energy Recharge +20%",
                "4": "Increases Elemental Burst DMG by 25% of Energy Recharge. A maximum of 75% bonus DMG can be obtained in this way."
            }
        },
        "resolutionOfSojourner": {
            "setName": "Resolution of Sojourner",
            "items": [
                "Heart of Comradeship",
                "Feather of Homecoming",
                "Sundial of the Sojourner",
                "Goblet of the Sojourner",
                "Crown of Parting"
            ],
            "effects": {
                "4": "Increases Charged Attack CRIT Rate by 30%.",
                "2": "ATK +18%."
            }
        },
        "braveHeart": {
            "setName": "Brave Heart",
            "items": [
                "Medal of the Brave",
                "Prospect of the Brave",
                "Fortitude of the Brave",
                "Outset of the Brave",
                "Crown of the Brave"
            ],
            "effects": {
                "4": "Increases DMG by 30% against opponents with more than 50% HP.",
                "2": "ATK +18%."
            }
        },
        "oceanHuedClam": {
            "setName": "Ocean-Hued Clam",
            "items": [
                "Sea-Dyed Blossom",
                "Deep Palace's Plume",
                "Cowry of Parting",
                "Pearl Cage",
                "Crown of Watatsumi"
            ],
            "effects": {
                "2": "Healing Bonus +15%.",
                "4": "When the character equipping this artifact set heals a character in the party, a Sea-Dyed Foam will appear for 3 seconds, accumulating the amount of HP recovered from healing (including overflow healing). At the end of the duration, the Sea-Dyed Foam will explode, dealing DMG to nearby opponents based on 90% of the accumulated healing. (This DMG is calculated similarly to Reactions such as Electro-Charged, and Superconduct, but is not affected by Elemental Mastery, Character Levels, or Reaction DMG Bonuses). Only one Sea-Dyed Foam can be produced every 3.5 seconds. Each Sea-Dyed Foam can accumulate up to 30,000 HP (including overflow healing). There can be no more than one Sea-Dyed Foam active at any given time. This effect can still be triggered even when the character who is using this artifact set is not on the field."
            }
        },
        "prayersForWisdom": {
            "setName": "Prayers for Wisdom",
            "items": [
                "Tiara of Thunder"
            ],
            "effects": {
                "1": "Affected by Electro for 40% less time."
            }
        },
        "travelingDoctor": {
            "setName": "Traveling Doctor",
            "items": [
                "Traveling Doctor's Silver Lotus",
                "Traveling Doctor's Owl Feather",
                "Traveling Doctor's Pocket Watch",
                "Traveling Doctor's Medicine Pot",
                "Traveling Doctor's Handkerchief"
            ],
            "effects": {
                "4": "Using Elemental Burst restores 20% HP.",
                "2": "Increases incoming healing by 20%."
            }
        },
        "adventurer": {
            "setName": "Adventurer",
            "items": [
                "Adventurer's Flower",
                "Adventurer's Tail Feather",
                "Adventurer's Pocket Watch",
                "Adventurer's Golden Goblet",
                "Adventurer's Bandana"
            ],
            "effects": {
                "4": "Opening a chest regenerates 30% Max HP over 5s.",
                "2": "Max HP increased by 1000."
            }
        },
        "heartOfDepth": {
            "setName": "Heart of Depth",
            "items": [
                "Gilded Corsage",
                "Gust of Nostalgia",
                "Copper Compass",
                "Goblet of Thundering Deep",
                "Wine-Stained Tricorne"
            ],
            "effects": {
                "4": "After using Elemental Skill, increases Normal Attack and Charged Attack DMG by 30% for 15s.",
                "2": "Hydro DMG Bonus +15%"
            }
        },
        "retracingBolide": {
            "setName": "Retracing Bolide",
            "items": [
                "Summer Night's Bloom",
                "Summer Night's Finale",
                "Summer Night's Moment",
                "Summer Night's Waterballoon",
                "Summer Night's Mask"
            ],
            "effects": {
                "2": "Increases Shield Strength by 35%.",
                "4": "While protected by a shield, gain an additional 40% Normal and Charged Attack DMG."
            }
        },
        "huskOfOpulentDreams": {
            "setName": "Husk of Opulent Dreams",
            "items": [
                "Bloom Times",
                "Plume of Luxury",
                "Song of Life",
                "Calabash of Awakening",
                "Skeletal Hat"
            ],
            "effects": {
                "2": "DEF +30%",
                "4": "A character equipped with this Artifact set will obtain the Curiosity effect in the following conditions: When on the field, the character gains 1 stack after hitting an opponent with a Geo attack, triggering a maximum of once every 0.3s. When off the field, the character gains 1 stack every 3s. Curiosity can stack up to 4 times, each providing 6% DEF and a 6% Geo DMG Bonus. When 6 seconds pass without gaining a Curiosity stack, 1 stack is lost."
            }
        },
        "lavaWalker": {
            "setName": "Lavawalker",
            "items": [
                "Lavawalker's Resolution",
                "Lavawalker's Salvation",
                "Lavawalker's Torment",
                "Lavawalker's Epiphany",
                "Lavawalker's Wisdom"
            ],
            "effects": {
                "4": "Increases DMG against opponents affected by Pyro by 35%.",
                "2": "Pyro RES increased by 40%."
            }
        },
        "crimsonWitch": {
            "setName": "Crimson Witch of Flames",
            "items": [
                "Witch's Flower of Blaze",
                "Witch's Ever-Burning Plume",
                "Witch's End Time",
                "Witch's Heart Flames",
                "Witch's Scorching Hat"
            ],
            "effects": {
                "4": "Increases Overloaded and Burning DMG by 40%. Increases Vaporize and Melt DMG by 15%. Using Elemental Skill increases the 2-Piece Set Bonus by 50% of its starting value for 10s. Max 3 stacks.",
                "2": "Pyro DMG Bonus +15%"
            }
        },
        "shimenawaReminiscence": {
            "setName": "Shimenawa's Reminiscence",
            "items": [
                "Entangling Bloom",
                "Shaft of Remembrance",
                "Morning Dew's Moment",
                "Hopeful Heart",
                "Capricious Visage"
            ],
            "effects": {
                "4": "When casting an Elemental Skill, if the character has 15 or more Energy, they lose 15 Energy and Normal/Charged/Plunging Attack DMG is increased by 50% for 10s. This effect will not trigger again during that duration.",
                "2": "ATK +18%."
            }
        },
        "blizzardStrayer": {
            "setName": "Blizzard Strayer",
            "items": [
                "Snowswept Memory",
                "Icebreaker's Resolve",
                "Frozen Homeland's Demise",
                "Frost-Weaved Dignity",
                "Broken Rime's Echo"
            ],
            "effects": {
                "4": "When a character attacks an opponent affected by Cryo, their CRIT Rate is increased by 20%. If the opponent is Frozen, CRIT Rate is increased by an additional 20%.",
                "2": "Cryo DMG Bonus +15%"
            }
        },
        "noblesseOblige": {
            "setName": "Noblesse Oblige",
            "items": [
                "Royal Flora",
                "Royal Plume",
                "Royal Pocket Watch",
                "Royal Silver Urn",
                "Royal Masque"
            ],
            "effects": {
                "2": "Elemental Burst DMG +20%",
                "4": "Using an Elemental Burst increases all party members' ATK by 20% for 12s. This effect cannot stack."
            }
        },
        "gambler": {
            "setName": "Gambler",
            "items": [
                "Gambler's Brooch",
                "Gambler's Feather Accessory",
                "Gambler's Pocket Watch",
                "Gambler's Dice Cup",
                "Gambler's Earrings"
            ],
            "effects": {
                "4": "Defeating an opponent has a 100% chance to remove Elemental Skill CD. Can only occur once every 15s.",
                "2": "Increases Elemental Skill DMG by 20%."
            }
        },
        "Empty": {
            "setName": "Gambler",
            "items": [
                "Gambler's Brooch",
                "Gambler's Feather Accessory",
                "Gambler's Pocket Watch",
                "Gambler's Dice Cup",
                "Gambler's Earrings"
            ],
            "effects": {
                "4": "Defeating an opponent has a 100% chance to remove Elemental Skill CD. Can only occur once every 15s.",
                "2": "Increases Elemental Skill DMG by 20%."
            }
        },
        "instructor": {
            "setName": "Instructor",
            "items": [
                "Instructor's Brooch",
                "Instructor's Feather Accessory",
                "Instructor's Pocket Watch",
                "Instructor's Tea Cup",
                "Instructor's Cap"
            ],
            "effects": {
                "2": "Increases Elemental Mastery by 80.",
                "4": "Upon triggering an Elemental Reaction, increases all party members' Elemental Mastery by 120 for 8s."
            }
        },
        "prayersForIllumination": {
            "setName": "Prayers for Illumination",
            "items": [
                "Tiara of Flame"
            ],
            "effects": {
                "1": "Affected by Pyro for 40% less time."
            }
        },
        "tinyMiracle": {
            "setName": "Tiny Miracle",
            "items": [
                "Tiny Miracle's Flower",
                "Tiny Miracle's Feather",
                "Tiny Miracle's Hourglass",
                "Tiny Miracle's Goblet",
                "Tiny Miracle's Earrings"
            ],
            "effects": {
                "4": "Incoming elemental DMG increases corresponding Elemental RES by 30% for 10s. Can only occur once every 10s.",
                "2": "All Elemental RES increased by 20%."
            }
        },
        "martialArtist": {
            "setName": "Martial Artist",
            "items": [
                "Martial Artist's Red Flower",
                "Martial Artist's Feather Accessory",
                "Martial Artist's Water Hourglass",
                "Martial Artist's Wine Cup",
                "Martial Artist's Bandana"
            ],
            "effects": {
                "2": "Increases Normal Attack and Charged Attack DMG by 15%.",
                "4": "After using Elemental Skill, increases Normal Attack and Charged Attack DMG by 25% for 8s."
            }
        },
        "prayersForDestiny": {
            "setName": "Prayers for Destiny",
            "items": [
                "Tiara of Torrents"
            ],
            "effects": {
                "1": "Affected by Hydro for 40% less time."
            }
        },
        "archaicPetra": {
            "setName": "Archaic Petra",
            "items": [
                "Flower of Creviced Cliff",
                "Feather of Jagged Peaks",
                "Sundial of Enduring Jade",
                "Goblet of Chiseled Crag",
                "Mask of Solitude Basalt"
            ],
            "effects": {
                "4": "Upon obtaining an Elemental Shard created through a Crystallize Reaction, all party members gain a 35% DMG Bonus for that particular element for 10s. Only one form of Elemental DMG Bonus can be gained in this manner at any one time.",
                "2": "Gain a 15% Geo DMG Bonus."
            }
        },
        "paleFlame": {
            "setName": "Pale Flame",
            "items": [
                "Stainless Bloom",
                "Wise Doctor's Pinion",
                "Moment of Cessation",
                "Surpassing Cup",
                "Mocking Mask"
            ],
            "effects": {
                "2": "Physical DMG is increased by 25%.",
                "4": "When an Elemental Skill hits an opponent, ATK is increased by 9% for 7s. This effect stacks up to 2 times and can be triggered once every 0.3s. Once 2 stacks are reached, the 2-set effect is increased by 100%."
            }
        },
        "scholar": {
            "setName": "Scholar",
            "items": [
                "Scholar's Bookmark",
                "Scholar's Quill Pen",
                "Scholar's Clock",
                "Scholar's Ink Cup",
                "Scholar's Lens"
            ],
            "effects": {
                "2": "Energy Recharge +20%",
                "4": "Gaining Elemental Particles or Orbs gives 3 Energy to all party members who have a bow or a catalyst equipped. Can only occur once every 3s."
            }
        },
        "luckyDog": {
            "setName": "Lucky Dog",
            "items": [
                "Lucky Dog's Clover",
                "Lucky Dog's Eagle Feather",
                "Lucky Dog's Hourglass",
                "Lucky Dog's Goblet",
                "Lucky Dog's Silver Circlet"
            ],
            "effects": {
                "4": "Picking up Mora restores 300 HP.",
                "2": "DEF increased by 100."
            }
        },
        "gladiatorFinale": {
            "setName": "Gladiator's Finale",
            "items": [
                "Gladiator's Nostalgia",
                "Gladiator's Destiny",
                "Gladiator's Longing",
                "Gladiator's Intoxication",
                "Gladiator's Triumphus"
            ],
            "effects": {
                "4": "If the wielder of this artifact set uses a Sword, Claymore or Polearm, increases their Normal Attack DMG by 35%.",
                "2": "ATK +18%."
            }
        },
        "prayersToSpringtime": {
            "setName": "Prayers to Springtime",
            "items": [
                "Tiara of Frost"
            ],
            "effects": {
                "1": "Affected by Cryo for 40% less time."
            }
        },
        "berserker": {
            "setName": "Berserker",
            "items": [
                "Berserker's Rose",
                "Berserker's Indigo Feather",
                "Berserker's Timepiece",
                "Berserker's Bone Goblet",
                "Berserker's Battle Mask"
            ],
            "effects": {
                "2": "CRIT Rate +12%",
                "4": "When HP is below 70%, CRIT Rate increases by an additional 24%."
            }
        },
        "tenacityOfTheMillelith": {
            "setName": "Tenacity of the Millelith",
            "items": [
                "Flower of Accolades",
                "Ceremonial War-Plume",
                "Orichalceous Time-Dial",
                "Noble's Pledging Vessel",
                "General's Ancient Helm"
            ],
            "effects": {
                "4": "When an Elemental Skill hits an opponent, the ATK of all nearby party members is increased by 20% and their Shield Strength is increased by 30% for 3s. This effect can be triggered once every 0.5s. This effect can still be triggered even when the character who is using this artifact set is not on the field.",
                "2": "HP increased by 20%."
            }
        },
        "defenderWill": {
            "setName": "Defender's Will",
            "items": [
                "Guardian's Flower",
                "Guardian's Sigil",
                "Guardian's Clock",
                "Guardian's Vessel",
                "Guardian's Band"
            ],
            "effects": {
                "2": "DEF +30%",
                "4": "For each different element present in your own party, the wearer's Elemental RES to that corresponding element is increased by 30%."
            }
        }
    },
    config: {
        w1: "Avg Effect Stack",
        w2: "Avg Effect Ratio",
        w3: "Recharge demand",
        w4: "Number",
        w5: "Refine",
        w6: "Enemy around",
        w7: "State",
        w8: "Effect1 Ratio",
        w9: "Effect2 Ratio",
        w10: "Team Energy Number Sum",
        w11: "「Nightstar」Stack",
        w12: "Hit in 0.3s",
        w13: "「Thunder Emblem」Stack",
        w14: "Full Stack Ratio",
        w15: "Shield Rate",
        w16: "「Recitative」Ratio",
        w17: "「Aria」Ratio",
        w18: "「Interlude」Ratio",
        w19: "Team Liyue Character Count",
        w20: "「Consummation」Avg Stack",
        w21: "Backend Ratio",
        w22: "At Least 2 Enemies Around",
        w23: "Full Stack Ratio",
        w24: "HP Below 50% Ratio",
        w25: "「Wavespike」Stack",
        w26: "「Mistsplitter\'s Emblem」Stack",
        w27: "EM",
        w28: "HP",
        w29: "Same Element Count",
        w30: "Diff Element Count",

        c1: "Talent「Harmony Between Heaven and Earth」Apply Ratio",
        c2: "Talent「Undivided Heart」Apply Ratio",
        c3: "HP Below 50%",
        c4: "Guide to Afterlife",
        c5: "Talent「Amatsumi Kunitsumi Sanctification」Apply Ratio",
        c6: "Talent「Kanten Senmyou Blessing」Apply Ratio",
        c7: "Kamisato Art: Senho",
        c8: "C6 Effect",
        c9: "Talent「Aristocratic Dignity」Apply Ratio",
        c10: "E附魔",
        c11: "HP <= 50%(Talent1: Healing Bonus + 15%)",
        c12: "Use C6",
        c13: "Talent「Strategic Reserve」Apply Ratio",
        c14: "Use Talent「Regina Probationum」",
        c15: "Electro Sigil Count",
        c16: "Talent「Hunger」Apply Ratio",
        c17: "Different Element Count",
        c18: "Talent「Tricks of the Trouble-Maker」Apply Ratio",
        c19: "Niwabi Enshou",
        c20: "Coil Stack",
        c21: "Under「Raging Oni King」",
        c22: "是否被大招附魔",
        c23: "Lightfall Sword Energy Stack",
        c24: "C6: Crimson Momiji",
        c25: "「Namisen」Stack",
        c26: "Under「Suiyuu」",
        c27: "After Q",
        c28: "Under Eye of Stormy Judgment",
        c29: "Chakra Desiderata Resolve",
        c30: "Under 「Ceremonial Garment」",
        c31: "Bane of All Evil",
        c32: "Talent「Conqueror of Evil: Tamer of Demons」Apply Stack",
        c33: "Talent「Dissolution Eon: Heaven Fall」Apply Stack",
        c34: "Use C4",
        c35: "「Sweeping Fervor」Shield Coverage",
        c36: "Brilliance",
        c37: "Off the Field",
        c38: "Talent「Keen Sight」Ratio",
        c39: "C2「Origins Known From the Stem」Ratio",
        c40: "C6「Sprinkling Weight」",
        c41: "「Golden Chalice's Bounty」Ratio",
        c42: "C2 Ratio",
        c43: "Prayer of the Crimson Crown",
        c44: "Pactsworn Pathclearer",
        c45: "Enable 「Judication」",
        c46: "C2「Ceremony: Homecoming of Spirits」Stacks",
        c47: "Q Pyro Bonus",
        c48: "Q Pyro Count",
        c49: "(C4) Schemes to Know‘s Seeds of Skandha Enemy Count",
        c50: "Under Windfavored State",
        c51: "Jade-Claimed Flower Hydro",
        c52: "Jade-Claimed Flower Pyro",
        c53: "Jade-Claimed Flower Cryo",
        c54: "Kuugoryoku Points",
        c55: "Prayerful Wind’s Benefit Ratio",
        c56: "Hurricane Guard effect Ratio",
        c57: "C4「Winsome」Ratio",
        c58: "C2「Debate」Stack",
        c59: "C4「Elucidation」Stack",
        c60: "C6「Structuration」Ratio",
        c61: "Chisel-Light Mirror",

        t1: "Type",
        t2: "Trigger Element",
        t3: "Skill",
        t4: "Recharge Requirement",
        t5: "Melt Ratio",
        t6: "Vaporize Ratio",
        t7: "Heal-DMG Weight(0: Pure Heal, 1: Pure DMG）",
        t8: "Swirl Frequency",
        t9: "Other's DMG Ratio",
        t10: "Swirl Frequency",
        t11: "Overload Frequency",
        t12: "Damage Requirement",
        t13: "Electro-charged Frequency",
        t14: "Overload Frequency",
        t15: "E-skill DMG Rate",
        t16: "Spread Ratio",
        t17: "Aggravate Ratio",
        t18: "Elemental Skill Times",
        t19: "Elemental Burst Times",
        t20: "Bloom Times",
        t21: "Teammates' equivalent EM",
        t22: "Teammates' Bloom Times",
        t23: "Combo",
        t24: "Attack Till Burst Expires",
        t25: "Electro-charge Ratio",
        t26: "Overload Ratio",
        t27: "Hyperbloom Ratio",
        t28: "Elemental Mastery Requirement",
        t29: "Buring Duration",
        t30: "Pryo Team Member Count",
        t31: "Addtional Normal SPD (exclude self and weapon)",
        t32: "SPD-DMG compensation",
        t33: "Gales of Reverie Wind Arrow Hits",
        t34: "Q Hits",
        t35: "Swirl Hits",
        t36: "Charged Hit",
        t37: "E Hit",
        t38: "Q Hit",
        t39: "Spread Ratio",

        a1: "Element",
        a2: "Effect Apply Ratio",
        a3: "Equivalent Crit Rate",
        a4: "Equivalent Stack",
        a5: "Avg Trigger Rate",
        a6: "「Curiosity」Equivalent Stack",
        a7: "Enemy Pyro Coverage",
        a8: "Effect1 Equivalent Stack",
        a9: "Full Stack Rate",
        a10: "Shield Coverage",
        a11: "Enemy Electro Coverage",
        a12: "Elemental Burst Rate",
        a13: "Avg Stack",
        a14: "Same Element Count",
        a15: "Different Element Count",

        p1: "ATK Valid",
        p2: "ATK Weight",
        p3: "ATK% Valid",
        p4: "ATK% Weight",
        p5: "HP Valid",
        p6: "HP Weight",
        p7: "HP% Valid",
        p8: "HP% Weight",
        p9: "DEF Valid",
        p10: "DEF Weight",
        p11: "DEF% Valid",
        p12: "DEF% Weight",
        p13: "Crit Rate Valid",
        p14: "Crit Rate Weight",
        p15: "Crit DMG Valid",
        p16: "Crit DMG Weight",
        p17: "EM Valid",
        p18: "EM Weight",
        p19: "Energy Recharge Valid",
        p20: "Energy Recharge Weight",

        b1: "Bannett Base ATK",
        b2: "C1",
        b3: "Skill Level",
        b4: "Gorou E Level",
        b5: "Domain Level",
        b6: "Swirl Element",
        b7: "Kazuha's EM",
        b8: "Ayato's Q Level",
        b9: "Sara Base ATK",
        b10: "C6",
        b11: "E Level",
        b12: "Q Level",
        b13: "C4",
        b14: "RaidenShogun E Level",
        b15: "Buffed Character's Max Energy",
        b16: "Rosaria's Crit Rate",
        b17: "Shenhe ATK",
        b18: "Shenhe E Level",
        b19: "Shenhe Q Level",
        b20: "C2",
        b21: "Hold or Press",
        b22: "Sucrose's EM",
        b23: "Swirl Type",
        b24: "Stack Level",
        b25: "Levitating",
        b26: "Element Transform Occurred",
        b27: "Transform Type",
        b28: "Seconds Passed",
        b29: "「Tricks of the Trouble-Maker」Stack",
        b30: "Yunjin DEF",
        b31: "60 Ascend",
        b32: "Different Element Count",
        b33: "Element",
        b34: "Apply Ratio",
        b35: "Effect① Ratio",
        b36: "Effect② Ratio",
        b37: "Crystallize Element",
        b38: "Triggered Burning, Bloom, Catalyze or Spread",
        b39: "HP Below 50%",
        b40: "Energy Below 50%",
        b41: "Apply Ratio",
        b42: "Nilou's HP",
        b43: "Candace's HP",
        b44: "Max EM in Team",
        b45: "Opponents Marked",
        b46: "Faruzan Base ATK",
        b47: "Perfidious Wind's Ruin Rate",
        b48: "Prayerful Wind's GiftRate",
        b49: "「Lost Wisdom of the Seven Caverns Rate（buffed hits count/total hit counts within a cycle）",
    },
    ele: {
        Pyro: "Pyro",
        Cryo: "Cryo",
        Hydro: "Hydro",
        Electro: "Electro",
        Anemo: "Anemo",
        Dendro: "Dendro",
        Geo: "Geo",
        Physical: "Physical",
        None: "None",
    },
    accountPage: {
        title: "Accounts and Syncing",
        syncButton: "Sync with a local directory",
        syncedButton: "Synced with the local directory",
        addAccount: "Add an account",
        chooseSyncBase: "Choose sync target",
        browserBase: "Based on the browser storage",
        fileBase: "Based on the local directory",
        lastModifiedAt: "last modified at ",
        confirmDelete: "Sure to delete?",
        delete: "delete",
        newAccountName: "New Account",
        deletingAccount: "Deleting the account",
        switchingAccount: "Switching accounts",
        cancelSyncing: "Synchronization canceled",
    },
    artPage: {
        shareDesc: "Use share link to import artifacts, expires in 1 day",
        deleteUnseen: "delete unseen artifacts",
        edit: "edit",
        recommend: "recommend",
        unlockAll: "unlock all",
        exportMona: "export Mona JSON",
        shareLink: "share link",
        confirmClear: "continue to clear? Will also clear artifact sets",
        monaJSON: "Mona JSON",
        show16: "Show only lvl > 16",
        copied: "Copied!",
        creating: "Creating",
        createDesc: "Mona is creating share link",
        wrongFormat: "Wrong format",
        importing: "Importing...",
        msg1: "Add a calculation preset to use this",
        newArt: "Add Artifact",
        dup: "Duplicate artifact detected, continue?",
    },
    kumiPage: {
        selectArt: "Select Artifact",
        newFolder: "New Folder",
        newKumi: "New Artifact Group",
        deleteFolder: "Delete Folder",
        enterFolder: "Please enter new folder name",
        renameFolder: "Rename Folder",
        enterKumi: "Please enter new group name",
        renameKumi: "Rename Artifact Group"
    },
    presetPage: {
        exportAll: "Export All",
        go: "Please go to",
        toCalc: "page to add new presets",
        wrongFormat: "Wrong format"
    },
    calcPage: {
        selectArt: "Select Artifact",
        dmgComp: "Damage Composition",
        selectBuff: "Select BUFF",
        setupCalc: "Computation Setup",
        algoDesc: "A*：Recommend<br>Heuristic Cutoff: Not ensure optimal result, but faster<br>Naive: Do not use",
        plzSetConst: "Please constraint artifact set or main stats, or it might be very time consuming, and leads to timeout",
        aStar: "A*",
        heuristic: "Heuristic Cutoff",
        naive: "Naive",
        constSet: "Constraint Artifact Set",
        constMain: "Constraint Main Stats",
        constMin: "Constraint Min Value",
        filKumi: "Filter Artifact Group",
        artAnalysis: "Artifacts Analysis",
        statCurve: "Stats Gain Curve",
        selectKumi: "Select Artifact Group",
        setupEnemy: "Enemy Setup",
        setupArt: "Artifact Effect Setup",
        artMode: "Artifact Effect Mode",
        modeAuto: "Auto",
        modeCustom: "Custom",
        artEffect: "Artifact Effects(Only works in custom mode)",
        start: "Optimize",
        newPreset: "New Preset",
        savePreset: "Save Preset",
        saveAsPreset: "Save as Preset",
        tfNormal: "Normal",
        beta: "BETA",
        statAnalysis: "Stat Analysis",
        saveKumi: "Save as Artifact Group",
        useKumi: "Use Artifact Group",
        effect4: "Set4 Effect:",
        dmg: "DMG Calculation",
        detail: "Detail",
        dmg2: "Transformative DMG",
        panel: "Panel",
        selectDir: "Select Dir",
        name: "Name",
        enterName: "Enter Name",
        bonus1: "1 Roll Gain", // todo
        statCount: "Roll Count", // todo
        gain: "Relative Gain",
        lockAll: "Lock All",
        unlockAll: "Unlock All",
        skill: "Skill",
        fumo: "附魔", // todo
        skillConfig: "Skill Config",
    },
    dmg: {
        electroCharged: "Electro-Charged",
        overload: "Overloaded",
        shattered: "Shattered",
        superConduct: "Superconduct",
        swirlElectro: "Swirl(Electro)",
        swirlPyro: "Swirl(Pyro)",
        swirlCryo: "Swirl(Cryo)",
        swirlHydro: "Swirl(Hydro)",
        burning: "Burning",
        crystallize: "Crystallize Shield",
        "Pyro": "Pyro DMG",
        "Hydro": "Hydro DMG",
        "Electro": "Electro DMG",
        "Cryo": "Cryo DMG",
        "Dendro": "Dendro DMG",
        "Geo": "Geo DMG",
        "Anemo": "Anemo DMG",
        "Physical": "Physical DMG",
        expect: "Expect DMG",
        crit: "Crit DMG",
        nonCrit: "Non Crit DMG",
        melt: "Melt",
        vaporize: "Vaporize",
        heal: "Regeneration",
        spread: "Spread",
        aggravate: "Aggravate",
        bloom: "Bloom",
        hyperbloom: "HyperBloom",
        burgeon: "Burgeon",
    },
    res: {
        Pyro: "Pyro Res",
        Cryo: "Cryo Res",
        Hydro: "Hydro Res",
        Electro: "Electro Res",
        Anemo: "Anemo Res",
        Dendro: "Dendro Res",
        Geo: "Geo Res",
        Physical: "Physical Res",
    },
    teamPage: {
        start: "Start",
        add: "Add Member",
        member: "Member",
        weight: "Weight",
        showStat: "Show Stats",
    },
    poPage: {
        start: "Start",
    },
    pfName: {
        ArtifactEff: "Artifact Eff",
    },
    pfDesc: {
        ArtifactEff: "1 point for a max-valued roll",
    },
    dbPage: {
        tooSmall: "Too little character data",
        weapon: "Weapon Usage",
        art: "Artifact Recommendation",
        mainStat: "Main Stat Recommendation",
        subStat: "Sub Stats Distribution Recommendation",
        count: "Rolls",
    },
    setupPage: {
        storage: "Storage",
        clear: "Clear local storage",
        confirmClear: "Confirm clear local storage?",
        loading: "Switching language",
    },
    skillType: {
        a: "Normal Attack",
        b: "Charged Attack",
        e: "Elemental Skill",
        q: "ElementalBurst",
    },
    buffGenre: {
        Character: "Character BUFF",
        Weapon: "Weapon BUFF",
        Artifact: "Artifact BUFF",
        Resonance: "Resonance",
        Common: "Custom",
    }
}
