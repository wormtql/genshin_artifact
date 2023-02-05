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
    tfName: {
        MaxATK: "Max ATK",
        MaxDEF: "Max DEF",
        MaxHP: "Max HP",
        MaxEM: "Max EM",
        MaxRecharge: "Max Recharge",
        PyroDamage: "Pyro DMG",
        CryoDamage: "Cryo DMG",
        HydroDamage: "Hydro DMG",
        ElectroDamage: "Electro DMG",
        AnemoDamage: "Anemo DMG",
        GeoDamage: "Geo DMG",
        PhysicalDamage: "Physical DMG",
        MaxVaporize: "Max Vaporize DMG",
        MaxMelt: "Max Melt DMG",
        ExpectVaporize: "Expect Vaporize DMG",
        ExpectMelt: "Expect Melt DMG",
        DendroDamage: "Dendro DMG",

        AlbedoDefault: "Albedo-Kreideprinz",
        AloyDefault: "Aloy-Savior From Another World",
        AmberDefault: "Amber-Outrider",
        AratakiIttoDefault: "Itto-The Evil",
        BarbaraDefault: "Barbara-Shining Idol",
        BeidouDefault: "Beidou-Uncrowned Lord of the Ocean",
        BennettDamage: "Bennett-Sub DPS",
        BennettDefault: "Bennett-Trial by Fire",
        ChongyunDefault: "Chongyun-Frozen Ardor",
        DilucDefault: "Diluc-Default",
        DionaDefault: "Diona-Kätzlein Cocktail",
        EulaDefault: "Eula-Spindrift Knight",
        FischlDefault: "Fischl-Prinzessin der Verurteilung",
        GanyuDefault: "Ganyu-Plenilune Gaze",
        GorouDefault: "Gorou-Canine Warrior",
        HuTaoDefault: "Hutao-Fragrance in Thaw",
        JeanDefault: "Jean-Dandelion Knight",
        KaedeharaKazuhaDamage: "Kazuha-DPS",
        KaedeharaKazuhaDefault: "Kazuha-Scarlet Leaves Pursue Wild Waves",
        KaeyaDefault: "Kaeya-Frostwind Swordsman",
        KamisatoAyakaDefault: "Ayaka-Frostflake Heron",
        KamisatoAyakaDps: "Ayaka-DPS with Recharge",
        KamisatoAyatoDefault: "Ayato-Pillar of Fortitude",
        KeqingDefault: "Keqing-Driving Thunder",
        KleeDefault: "Klee-Fleeing Sunlight",
        KujouSaraDamage: "Kujou Sara-Sub DPS",
        KujouSaraDefault: "Kujou Sara-Crowfeather Kaburaya",
        LisaDefault: "Lisa-Witch of Purple Rose",
        MonaDefault: "Mona-Astral Reflection",
        NingguangDefault: "Ningguang-Eclipsing Star",
        NoelleDefault: "Noelle-Chivalric Blossom",
        QiqiDefault: "Qiqi-Icy Resurrection",
        RaidenShogunDefault: "Raiden Shogun-Plane of Euthymia",
        RazorDefault: "Razor-Default",
        RosariaDefault: "Rosaria-Thorny Benevolence",
        SangonomiyaKokomiDefault: "Kokomi-Pearl of Wisdom",
        SayuDefault: "Sayu-Mujina Ninja",
        ShenheDefault: "Shenhe-Lonesome Transcendence",
        SucroseDefault: "Sucrose-Harmless Sweetie",
        TartagliaDefault: "Tartaglia-「Childe」",
        ThomaDefault: "Thoma-Protector From Afar",
        VentiDefault: "Venti-Windborne Bard",
        XianglingDefault: "Xiangling-Exquisite Delicacy",
        XiaoDefault: "Xiao-Vigilant Yaksha",
        XingqiuDefault: "Xingqiu-Juvenile Galant",
        XinyanDamage: "Xinyan-DPS",
        XinyanDefault: "Xinyan-Blazing Riff",
        YaeMikoDefault: "Yae-Astute Amusement",
        YanfeiDefault: "Yanfei-Wise Innocence",
        YelanDefault: "Yelan-Valley Orchid",
        YoimiyaDefault: "Yoimiya-Frolicking Flames",
        YunjinDefault: "Yunjin-Stage Lucida",
        ZhongliDefault: "Zhongli-Vago Mundo",
        KukiShinobuDefault: "Shinobu-Mender of Tribulations",
        ShikanoinHeizouDefault: "Shikanoin Heizou-Analytical Harmony",
        TighnariDefault: "Tighnari-Verdant Strider",
        NilouDefault: "Nilou-Dance of Lotuslight",
        CynoDefault: "Cyno-Judicator of Secrets",
        NahidaDefault: "Nahida-Physic of Purity",
        WandererDefault: "Wanderer-Eons Adrift",
        FaruzanDamage: "Faruzan-Enigmatic Machinist",
        AlhaithamDefault: "Alhaitham-Admonishing Instruction",
    },
    tfDesc: {
        MaxATK: "Maximize ATK",
        MaxDEF: "Maximize DEF",
        MaxHP: "Maximize HP",
        MaxEM: "Maximize Elemental Mastery",
        MaxRecharge: "Maximize Energy Recharge",
        PyroDamage: "Maximize Crit or Avg Pyro Damage",
        CryoDamage: "Maximize Crit or Avg Cryo Damage",
        HydroDamage: "Maximize Crit or Avg Hydro Damage",
        ElectroDamage: "Maximize Crit or Avg Electro Damage",
        AnemoDamage: "Maximize Crit or Avg Anemo Damage",
        GeoDamage: "Maximize Crit or Avg Geo Damage",
        PhysicalDamage: "Maximize Crit or Avg Physical Damage",
        MaxVaporize: "Maximize vaporize DMG<br><b>Attention:</b>This function only calculates the simplest case, some attribute conversions are not considered, you may not use this unless you know what you're doing",
        MaxMelt: "Maximize vaporize DMG<br><b>Attention:</b>This function only calculates the simplest case, some attribute conversions are not considered, you may not use this unless you know what you're doing",
        ExpectVaporize: "Maximize vaporize DMG<br><b>Attention:</b>This function only calculates the simplest case, some attribute conversions are not considered, you may not use this unless you know what you're doing",
        ExpectMelt: "Maximize vaporize DMG<br><b>Attention:</b>This function only calculates the simplest case, some attribute conversions are not considered, you may not use this unless you know what you're doing",
        DendroDamage: "Maximize Crit or Avg Dendro Damage",

        AlbedoDefault: "Sub DPS Albedo",
        AloyDefault: "DPS Aloy",
        AmberDefault: "DPS Amber",
        AratakiIttoDefault: "DPS Itto",
        BarbaraDefault: "Miximize Barbara's Q regeneration",
        BeidouDefault: "普通北斗弹反流",
        BennettDamage: "Sub DPS Bennett",
        BennettDefault: "Support Bennett",
        ChongyunDefault: "Sub DPS Chongyun",
        DilucDefault: "DPS Diluc",
        DionaDefault: "Healing, Shield Support",
        EulaDefault: "DPS Eula",
        FischlDefault: "Elemental DPS Fischl",
        GanyuDefault: "DPS Ganyu",
        GorouDefault: "Support Gorou",
        HuTaoDefault: "Main DPS Hutao",
        JeanDefault: "Jean Default",
        KaedeharaKazuhaDamage: "DPS Kazuha(also support)",
        KaedeharaKazuhaDefault: "Support Kazuha",
        KaeyaDefault: "Cryo DPS Kaeya",
        KamisatoAyakaDefault: "Main DPS Ayaka",
        KamisatoAyakaDps: "DPS Ayaka, with recharge into consideration, Simulation: 4s 左右辅助铺场，平a4段接重击，有e放e，有大开大",
        KamisatoAyatoDefault: "DPS Ayato",
        KeqingDefault: "Electro DPS Keqing",
        KleeDefault: "Pyro DPS Klee",
        KujouSaraDamage: "Sub DPS Sara",
        KujouSaraDefault: "Support Sara",
        LisaDefault: "DPS Lisa",
        MonaDefault: "DPS Mona",
        NingguangDefault: "DPS Ningguang",
        NoelleDefault: "DPS Noelle",
        QiqiDefault: "Support Qiqi",
        RaidenShogunDefault: "DPS Shogun",
        RazorDefault: "Physical DPS Razor",
        RosariaDefault: "Support Rosaria with a certain amount of DPS",
        SangonomiyaKokomiDefault: "DPS Kokomi",
        SayuDefault: "DPS Sayu",
        ShenheDefault: "Support Shenhe",
        SucroseDefault: "Support Sucrose",
        TartagliaDefault: "DPS Tartaglia",
        ThomaDefault: "Shield Support Thoma",
        VentiDefault: "DPS Venti",
        XianglingDefault: "Cryo DPS Xiangling",
        XiaoDefault: "DPS Xiao",
        XingqiuDefault: "Sub DPS Xingqiu",
        XinyanDamage: "Physical DPS Xinyan",
        XinyanDefault: "Support Xinyan",
        YaeMikoDefault: "DPS Yae Miko",
        YanfeiDefault: "DPS Yanfei",
        YelanDefault: "DPS Yelan, maximizing Q damage",
        YoimiyaDefault: "DPS Yoimiya",
        YunjinDefault: "Support Yunjin",
        ZhongliDefault: "DPS Zhongli",
        KukiShinobuDefault: "输出型久岐忍。使其大招和越祓雷草之轮伤害按一定比例之和最大",
        ShikanoinHeizouDefault: "DPS Heizou, maximizing E Stack-4 DMG",
        TighnariDefault: "Maximize Tighnari Charged Attack",
        NilouDefault: "Nilou DPS",
        CynoDefault: "Perform QTE and unleash Duststalker Bolts, best combo would be applied automatically basing on artifact set",
        NahidaDefault: "Maximize Nahida's Tri-Karma Purification DMG",
        WandererDefault: "Maximize Wanderer's total damage in one cycle",
        FaruzanDamage: "Maximize Faruzan's Q damage",
        AlhaithamDefault: "Charged DMG, 1-Mirror Projection Attack DMG, Single-Instance DMG, multiplied by their counts respectively",
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
    buffName: {
        ATKFixed: "ATK",
        ATKPercentage: "ATK%",
        Critical: "Crit Rate",
        CriticalDamage: "Crit DMG",
        CustomBonus: "DMG Bonus",
        DEFFixed: "DEF",
        DEFMinus: "减防",
        DEFPercentage: "DEF%",
        ElementalMastery: "Elemental Mastery",
        HealingBonus: "Healing Bonus",
        HPFixed: "HP",
        HPPercentage: "HP%",
        Recharge: "Energy Recharge",
        ResMinus: "减抗",
        AlbedoTalent2: "Albedo-「Homuncular Nature」",
        AlbedoC4: "Albedo-「Descent of Divinity」",
        AlbedoC6: "Albedo-「Dust of Purification」",
        AloyTalent1: "Aloy-「Combat Override」",
        AratakiIttoC4: "Itto-「Jailhouse Bread and Butter」",
        BeidouC6: "Beidou-「Bane of Evil」",
        BennettQ: "Bennett-「Fantastic Voyage」",
        BennettC6: "Bennett-「Fire Ventures With Me」",
        ChongyunTalent2: "Chongyun-「Rimechaser Blade」",
        DionaC6G50: "Diona-「Cat's Tail Closing Time」",
        EulaE: "Eula-「Icetide Vortex」减抗",
        GanyuTalent2: "Ganyu-「Harmony Between Heaven and Earth」",
        GanyuC1: "Ganyu-「Dew-Drinker」",
        GorouE1: "Gorou-「General's War Banner」-1",
        GorouE3: "Gorou-「General's War Banner」-3",
        GorouTalent1: "Gorou-「Heedless of the Wind and Weather」",
        GorouC6: "Gorou-「Valiant Hound: Mountainous Fealty」",
        HuTaoTalent1: "Hutao-「Flutter By」",
        JeanC4: "Jean-「Lands of Dandelion」",
        KaedeharaKazuhaTalent2: "Kazuha-「Poetics of Fuubutsu」",
        KaedeharaKazuhaC2: "Kazuha-「Yamaarashi Tailwind」",
        KamisatoAyakaC4: "Ayaka-「Ebb and Flow」",
        KamisatoAyatoQ: "Ayato-「Suiyuu」",
        KleeC2: "Klee-「Explosive Frags」",
        KleeC6: "Klee-「Blazing Delight」",
        KujouSaraEOrQ: "KujouSara-「Tengu Juurai」",
        LisaTalent2: "Lisa-「Static Electricity Field」",
        MonaQ: "Mona-「Omen」",
        MonaC1: "Mona-「Prophecy of Submersion」",
        NingguangTalent2: "Ningguang-「Strategic Reserve」",
        RaidenShogunE: "Raiden Shogun-「Eye of Stormy Judgment」",
        RaidenShogunC4: "Raiden Shogun-「Pledge of Propriety」",
        RazorC4: "Razor-「Bite」",
        RosariaTalent2: "Rosaria-「Shadow Samaritan」",
        RosariaC6: "Rosaria-「Divine Retribution」",
        ShenheE: "Shenhe-「Icy Quill」",
        ShenheQ: "Shenhe-「Divine Maiden's Deliverance」减抗",
        ShenheTalent1: "Shenhe-「Deific Embrace」",
        ShenheTalent2: "Shenhe-「Spirit Communion Seal」",
        SucroseTalent1: "Sucrose-「Catalyst Conversion」",
        SucroseTalent2: "Sucrose-「Mollis Favonius」",
        SucroseC6: "Sucrose-「Chaotic Entropy」",
        ThomaTalent1: "Thoma-「Imbricated Armor」",
        ThomaC6: "Thoma-「Burning Heart」",
        VentiC2: "Venti-「Breeze of Reminiscence」",
        VentiC6: "Venti-「Storm of Defiance」",
        XianglingTalent2: "Xiangling-「Beware, It's Super Hot!」",
        XianglingC1: "Xiangling-「Crispy Outside, Tender Inside」",
        XianglingC6: "Xiangling-「Condensed Pyronado」",
        XingqiuC2: "Xingqiu-「Rainbow Upon the Azure Sky」",
        XinyanC4: "Xinyan-「Wildfire Rhythm」",
        XinyanTalent2: "Xinyan-「Now That's Rock 'N' Roll!」",
        YaeMikoC4: "Yae-「Sakura Channeling」",
        YelanTalent2: "Yelan-「Adapt With Ease」",
        YelanC4: "Yelan-「Bait-and-Switch」",
        YoimiyaTalent2: "Yoimiya-「Summer Night's Dawn」",
        YunjinQ: "Yunjin-「Flying Cloud Flag Formation」",
        YunjinC2: "Yunjin-「Myriad Mise-En-Scène」",
        ZhongliShield: "Zhongli-「Jade Shield」",
        ElegyOfTheEnd: "Elegy for the End-「Millennial Movement: Farewell Song」",
        HakushinRing: "Hakushin Ring-「Sakura Saiguu」",
        ThrillingTalesOfDragonSlayers: "Thrilling Tales of Dragon Slayers-「Heritage」",
        SongOfBrokenPines: "Song of Broken Pines-「Millennial Movement: Banner-Hymn」",
        WolfsGravestone: "Wolf's Gravestone-「Wolfish Tracker」",
        FreedomSworn: "Freedom-Sworn-「Millennial Movement: Song of Resistance」",
        ResonancePyro2: "Resonance-Fervent Flames",
        ResonanceCryo2: "Resonance-Shattering Ice",
        ResonanceGeo2: "Resonance-Enduring Rock",
        ArchaicPetra4: "Archaic Petra 4",
        Instructor4: "Instructor 4",
        NoblesseOblige4: "Noblesse Oblige 4",
        TenacityOfTheMillelith4: "Tenacity of the Millelith 4",
        ViridescentVenerer4: "Viridescent Venerer 4",
        ShikanoinHeizouTalent2: "Shikanoin Heizou-Penetrative Reasoning",
        TighnariC4: "Tighnari-「Withering Glimpsed in the Leaves」",
        DoriC4: "Dori-「Discretionary Supplement」",
        SapwoodBlade: "Sapwood Blade/Forest Regalia-「Leaf of Consciousness」",
        Moonpiercer: "Moonpiercer-「Leaf of Revival」",
        BaseDmg: "Base DMG",
        ResonanceHydro2: "Resonance-Soothing Water",
        ResonanceDendro2: "Resonance-Sprawling Greenery",
        XiphosMoonlight: "Xiphos’ Moonlight-「Whisper of the Jinn」",
        MakhairaAquamarine: "Makhaira Aquamarine-「Desert Pavilion」",
        KeyOfKhajNisut: "Key of Khaj-Nisut-「Sunken Song of the Sands」",
        NilouTalent1: "Nilou-「Court of Dancing Petals」",
        NilouTalent2: "Nilou-「Dreamy Dance of Aeons」",
        CandaceQ: "Candace-「Prayer of the Crimson Crown」",
        CandaceTalent2: "Candace-「Celestial Dome of Sand」",
        NahidaTalent1: "Nahida-「Compassion Illuminated」",
        FaruzanQ: "Faruzan-「The Wind’s Secret Ways」",
    },
    buffDesc: {
        ATKFixed: "",
        ATKPercentage: "",
        Critical: "",
        CriticalDamage: "",
        CustomBonus: "",
        DEFFixed: "",
        DEFMinus: "",
        DEFPercentage: "",
        ElementalMastery: "",
        HealingBonus: "",
        HPFixed: "",
        HPPercentage: "",
        Recharge: "",
        ResMinus: "",
        AlbedoTalent2: "Albedo Talent2: Using Rite of Progeniture: Tectonic Tide increases the Elemental Mastery of nearby party members by 125 for 10s.",
        AlbedoC4: "Albedo C4: Active party members within the Solar Isotoma field have their Plunging Attack DMG increased by 30%.",
        AlbedoC6: "Albedo C6: Active party members within the Solar Isotoma field who are protected by a shield created by Crystallize have their DMG increased by 17%.",
        AloyTalent1: "Aloy Talent1: When Aloy receives the Coil effect from Frozen Wilds, nearby party members' ATK is increased by 8%. This effect lasts 10s.",
        AratakiIttoC4: "Itto C4: When the Raging Oni King state caused by Royal Descent: Behold, Itto the Evil! ends, all nearby party members gain 20% DEF and 20% ATK for 10s.",
        BeidouC6: "Beidou C6: 斫雷持续期间，周围敌人的雷元素抗性降低15%。",
        BennettQ: "Bennett Q: 基于班尼特的基础攻击力，以一定比例获得攻击力加成<br>一命：美妙旅程的攻击力提升效果不再有血量限制，数值上追加班尼特基础攻击力的20%。",
        BennettC6: "Bennett C6: 处在美妙旅程领域内的队伍中当前场上单手剑、双手剑、长柄武器角色获得15%火元素伤害加成<br>注：此处不管当前角色的武器类型",
        ChongyunTalent2: "Chongyu Talent2: When the field created by Spirit Blade: Chonghua's Layered Frost disappears, another spirit blade will be summoned to strike nearby opponents, dealing 100% of Chonghua's Layered Frost's Skill DMG as AoE Cryo DMG. Opponents hit by this blade will have their Cryo RES decreased by 10% for 8s.",
        DionaC6G50: "Diona C6: Elemental Mastery increased by 200 when HP is above 50%.",
        EulaE: "Eula E: 长按若消耗了冷酷之心效果，会使身边的敌人的物理抗性与冰元素抗性降低。",
        GanyuTalent2: "甘雨天赋2：降众天华领域内的队伍中当前场上角色获得20%冰元素伤害加成。",
        GanyuC1: "甘雨命座1：二段蓄力重击的霜华矢或霜华绽发命中敌人时，会使敌人的冰元素抗性降低15%，持续6秒。",
        GorouE1: "五郎E技能：一名角色时：「坚牢」：防御力提升。",
        GorouE3: "五郎E技能：三名角色时：「摧碎」：除上述效果外，获得岩元素伤害加成。",
        GorouTalent1: "五郎天赋1：施放兽牙逐突形胜战法后的12秒内，附近的队伍中所有角色的防御力提升25%。",
        GorouC6: "五郎命座6：施放犬坂吠吠方圆阵或兽牙逐突形胜战法后的12秒内，依据施放时的领域等级，提高附近的队伍中所有角色岩元素伤害的暴击伤害。",
        HuTaoTalent1: "胡桃天赋1：蝶引来生施加的彼岸蝶舞状态结束后，队伍中所有角色（不包括胡桃自己）的暴击率提高12%，持续8秒。",
        JeanC4: "琴命座4：在蒲公英之风的领域内，所有敌人的风元素抗性下降40％。",
        KaedeharaKazuhaTalent2: "枫原万叶天赋2：枫原万叶触发扩散反应后，枫原万叶的每点元素精通，会为队伍中所有角色提供0.04%对应元素伤害加成，持续8秒。",
        KaedeharaKazuhaC2: "枫原万叶命座2：场上角色的元素精通提升200点。",
        KamisatoAyakaC4: "绫华命座4：敌人受到神里流•霜灭的霜见雪关扉造成的伤害后，防御力降低30%，持续6秒。",
        KamisatoAyatoQ: "神里绫人Q技能：展开清净之园囿，熄灭其中一切嚣闹。存在期间，其中会持续降下水花剑，攻击范围内的敌人，造成水元素伤害，并提高其中的角色的普通攻击伤害。",
        KleeC2: "可莉命座2：蹦蹦炸弹的诡雷会使敌人防御力降低23％，持续10秒。",
        KleeC6: "可莉命座6：施放轰轰火花后的25秒内，队伍中所有角色获得10％火元素伤害加成。",
        KujouSaraEOrQ: "九条裟罗E/Q技能：基于九条裟罗的基础攻击力，以一定比例获得攻击力加成<br>六命：处于天狗咒雷带来的攻击力提升效果状态下的角色，其雷元素伤害的暴击伤害提高60%。",
        LisaTalent2: "丽莎天赋2：敌人受到蔷薇的雷光攻击后，降低15%防御力，持续10秒。",
        MonaQ: "莫娜Q技能：对敌人施加星异的伤害加成效果，并以此提高这一次造成的伤害。四命：队伍中所有角色攻击处于星异状态下的敌人时，暴击率提升15%",
        MonaC1: "莫娜命座1：队伍中自己的角色攻击命中处于星异状态下的敌人后的8秒内，水元素相关反应的效果提升：<br>•感电反应造成的伤害提升15%，蒸发反应造成的伤害提升15%，水元素扩散反应造成的伤害提升15%",
        NingguangTalent2: "凝光天赋2：穿过璇玑屏的角色会获得12%岩元素伤害加成，持续10秒。",
        RaidenShogunE: "雷电将军E技能：雷罚恶曜之眼的角色在持续期间内，元素爆发造成的伤害获得提升，提升程度基于元素爆发的元素能量。",
        RaidenShogunC4: "雷电将军命座4：奥义•梦想真说施加的梦想一心状态结束后，附近的队伍中所有角色（不包括雷电将军自己）的攻击力提升30%，持续10秒。",
        RazorC4: "雷泽命座4：利爪与苍雷点按时，会使命中的敌人防御力降低15％，持续7秒。",
        RosariaTalent2: "罗莎莉亚天赋2：施放终命的圣礼时，基于自身暴击率的15%，提高附近的队伍中所有角色(不包括罗莎莉亚自己)的暴击率，持续10秒。通过这种方式获得的暴击率提升，无法超过15%。",
        RosariaC6: "罗莎莉亚命座6：终命的圣礼的攻击会使敌人的物理抗性降低20%，持续10秒。",
        ShenheE: "申鹤E技能：基于申鹤自己当前的攻击力，提高造成的伤害。",
        ShenheQ: "申鹤Q技能：「箓灵」将结成领域，使其中敌人的冰元素抗性与物理抗性降低。",
        ShenheTalent1: "申鹤天赋1：处于神女遣灵真诀的领域中的当前场上角色，冰元素伤害加成提高15%。二命：领域中的当前场上角色，冰元素伤害的暴击伤害提高15%。",
        ShenheTalent2: "申鹤天赋2：申鹤施放仰灵威召将役咒后，将使附近的队伍中所有角色获得如下效果：<br>•点按：元素战技和元素爆发造成的伤害提高15%，持续10秒；<br>•长按：普通攻击、重击和下落攻击造成的伤害提高15%，持续15秒。",
        SucroseTalent1: "砂糖天赋1：砂糖触发扩散反应时，使队伍中所有对应元素类型的角色（不包括砂糖自己）元素精通提升50，持续8秒。",
        SucroseTalent2: "砂糖天赋2：风灵作成·陆叁零捌或禁·风灵作成·染伍同构贰型命中敌人时，基于砂糖元素精通的20%,为队伍中所有角色（不包括砂糖自己）提供元素精通加成，持续8秒。",
        SucroseC6: "砂糖命座6：禁·风灵作成·柒伍同构贰型如果发生了元素转化，则使队伍中所有角色在技能持续时间内获得20%的对应元素伤害加成。",
        ThomaTalent1: "托马天赋1：当前场上自己的角色获取或刷新烈烧佑命护盾时，护盾强效提升5%，持续时间6秒。此效果每0.3秒至多触发一次，至多叠加5次。",
        ThomaC6: "托马命座6：获取或刷新烈烧佑命护盾时，队伍中所有角色的普通攻击，重击与下落攻击造成的伤害提升15%，持续6秒。",
        VentiC2: "温迪命座2：高天之歌会使敌人的风元素抗性与物理抗性降低12％，持续10秒。被高天之歌击飞的敌人在落地前，风元素抗性与物理抗性额外降低12％。",
        VentiC6: "温迪命座6：温迪六命BUFF。受风神之诗伤害的敌人，风元素抗性降低20％。若产生了元素转化，则使转换的元素抗性也降低20％。",
        XianglingTalent2: "香菱天赋2：锅巴出击效果结束时，锅巴会在消失的位置留下辣椒。拾取辣椒会提高10%攻击力，持续10秒。",
        XianglingC1: "香菱命座1：受到锅巴攻击的敌人，火元素抗性降低15％，持续6秒。",
        XianglingC6: "香菱命座6；旋火轮持续期间，队伍中所有角色获得15％火元素伤害加成。",
        XingqiuC2: "行秋命座2：受到剑雨攻击的敌人，水元素抗性降低15％，持续4秒。",
        XinyanC4: "辛焱命座4：热情拂扫的伤害，会使敌人的物理抗性降低15%，持续12秒。",
        XinyanTalent2: "辛焱天赋2：处于热情拂扫的护盾保护下的角色造成的物理伤害提高15%。",
        YaeMikoC4: "八重神子命座4：杀生樱的落雷命中敌人后，队伍中附近的所有角色获得20%雷元素伤害加成，持续5秒。",
        YelanTalent2: "夜兰天赋2：「玄掷玲珑」存在期间，能使队伍中自己的当前场上角色造成的伤害提高1%，并且每1秒进一步提高3.5%，至多使角色造成的伤害提高50%。效果存在期间重新施放渊图玲珑骰，将移除原有的上述效果。",
        YelanC4: "夜兰4命：依照「络命丝」标记敌人的数量，每次标记将在爆发时使队伍中所有角色的生命值上限提升10%，持续25秒。通过这种方式，生命值上限至多获得40%提升。",
        YoimiyaTalent2: "宵宫天赋2：释放琉金云间草后的15秒内，附近的队伍中所有其它角色（不包括宵宫自己）攻击力提高10%。此外，依据宵宫自己释放琉金云间草时固有天赋「袖火百景图」的叠加层数，将额外提升上述的攻击力效果，每层提升1%攻击力。",
        YunjinQ: "云堇Q技能：对敌人造成普通攻击伤害时，基于云堇自己当前的防御力，提高造成的伤害。<br>天赋「莫从恒蹊」：「飞云旗阵」提供的普通攻击伤害提高，当队伍中存在1/2/3/4种元素类型的角色时，数值上进一步追加云堇防御力的2.5%/5.0%/7.5%/11.5%。",
        YunjinC2: "云堇命座2：施放破嶂见旌仪后，附近队伍中所有角色普通攻击造成的伤害提高15%，持续12秒。",
        ZhongliShield: "钟离盾：使附近小范围内敌人的所有元素抗性与物理抗性降低20%",
        ElegyOfTheEnd: "千年的大乐章·别离之歌」效果：元素精通提高100/125/150/175/200点，攻击力提升20%/25%/30%/35%/40%。",
        HakushinRing: "樱之斋宫：装备该武器的角色触发雷元素相关反应后，队伍中附近的与该元素反应相关的元素类型的角色，获得10%/12.5%/15%/17.5%/20%对应元素的元素伤害加成，持续6秒。通过这种方式，角色获得的元素伤害加成无法叠加。",
        ThrillingTalesOfDragonSlayers: "传承：主动切换角色时，新登场的角色攻击力提升24%/30%/36%/42%/48%，持续10秒。该效果每20秒只能触发一次。",
        SongOfBrokenPines: "「千年的大乐章·揭旗之歌」效果：普通攻击速度提升12%/15%/18%/21%/24%，攻击力提升20%/25%/30%/35%/40%。",
        WolfsGravestone: "攻击命中生命值低于30%的敌人时，队伍中所有成员的攻击力提高40%/50%/60%/70%/80%，持续12秒。该效果30秒只能触发一次。",
        FreedomSworn: "「千年的大乐章·抗争之歌」效果：普通攻击、重击、下落攻击造成的伤害提升16%/20%/24%/28%/32%，攻击力提升20%/25%/30%/35%/40%。",
        ResonancePyro2: "攻击力提升25%。",
        ResonanceCryo2: "攻击冰元素附着或冻结状态下的敌人时，暴击率提高15%。",
        ResonanceGeo2: "护盾强效提升15%。角色处于护盾保护状态时，①造成的伤害提升15%，对敌人造成伤害时会使敌人的的②岩元素抗性降低20%，持续15秒。",
        ArchaicPetra4: "获得结晶反应形成的晶片时，队伍中所有角色获得35%对应元素伤害加成，持续10秒。",
        Instructor4: "触发元素反应后。队伍中所有角色元素精通提高120点，持续8秒。",
        NoblesseOblige4: "施放元素爆发后，队伍中所有角色攻击力提升20％，持续12秒。该效果不可叠加。",
        TenacityOfTheMillelith4: "元素战技命中敌人后，使队伍中附近的所有角色攻击力提升20%，护盾强效提升30%，持续3秒。",
        ViridescentVenerer4: "根据扩散的元素类型，降低受到影响的敌人40%的对应元素抗性，持续10秒。",
        ShikanoinHeizouTalent2: "Shikanoin Heizou Talent: After Shikanoin Heizou's Heartstopper Strike hits an opponent, increases all party members' (excluding Shikanoin Heizou) Elemental Mastery by 80 for 10s.",
        TighnariC4: "Tighnari C4: When Fashioner’s Tanglevine Shaft is unleashed, all nearby party members gain 60 Elemental Mastery for 8s. If the Fashioner’s Tanglevine Shaft triggers a Burning, Bloom, Aggravate, or Spread reaction, their Elemental Mastery will be further increased by 60. This latter case will also refresh the buff state’s duration.",
        DoriC4: "Dori C4: The character connected to the Jinni will obtain the following buffs based on their current HP and Energy:<br>·When their HP is lower than 50%, they gain 50% Incoming Healing Bonus.<br>·When their Energy is less than 50%, they gain 30% Energy Recharge.",
        SapwoodBlade: "When picked up, the Leaf will grant the character 60 Elemental Mastery for 12s",
        Moonpiercer: "When picked up, the Leaf will grant the character 16% ATK for 12s",
        BaseDmg: "Flat DMG",
        ResonanceHydro2: "", // todo
        ResonanceDendro2: "", // todo
        XiphosMoonlight: "The equipping character will gain 0.036%/0.045%/0.054%/0.063%/0.072% Energy Recharge for each point of Elemental Mastery they possess for 12s, with nearby party members gaining 30% of this buff for the same duration. Multiple instances of this weapon can allow this buff to stack. This effect will still trigger even if the character is not on the field.",
        MakhairaAquamarine: "The equipping character will gain 24%/30%/36%/42%/48% of their Elemental Mastery as bonus ATK for 12s, with nearby party members gaining 30% of this buff for the same duration. Multiple instances of this weapon can allow this buff to stack. This effect will still trigger even if the character is not on the field.",
        KeyOfKhajNisut: "The Elemental Mastery of all nearby party members will be increased by 0.2%/0.25%/0.3%/0.35%/0.4% of the equipping character’s max HP for 20s.",
        NilouTalent1: "Characters under the effect of Golden Chalice’s Bounty will increase the Elemental Mastery of all nearby characters by 100 for 10s whenever they are hit by Dendro attacks",
        NilouTalent2: "Each 1,000 points of Nilou’s Max HP above 30,000 will cause the DMG dealt by Bountiful Cores created by characters affected by Golden Chalice’s Bounty to increase by 7%.<br>The maximum increase in Bountiful Core DMG that can be achieved this way is 300%.",
        CandaceQ: "Characters deal increased Elemental DMG with their Normal Attacks(20%)",
        CandaceTalent2: "Characters affected by the Prayer of the Crimson Crown caused by Sacred Rite: Wagtail’s Tide will deal 0.5% increased DMG to opponents for every 1,000 points of Candace’s Max HP when they deal Elemental DMG with their Normal Attacks.",
        NahidaTalent1: "When unleashing Illusory Heart, the Shrine of Maya will gain the following effects:<br>The Elemental Mastery of the active character within the field will be increased by 25% of the Elemental Mastery of the party member with the highest Elemental Mastery. You can gain a maximum of 250 Elemental Mastery in this manner.",
        FaruzanQ: "Faruzan Elemental Burst: <br>·When the Whirlwind Pulse hits opponents, it will apply Perfidious Wind's Ruin to them, decreasing their Anemo RES.<br>·The Whirlwind Pulse will also apply Prayerful Wind's Gift to all nearby characters when it is unleashed, granting them Anemo DMG Bonus.<br>When characters affected by The Wind's Secret Ways' Prayerful Wind's Gift deal Anemo DMG to opponents, this DMG will be increased based on 32% of Faruzan's own ATK. This DMG Bonus will be cleared 0.1s after dealing Anemo DMG to opponents, and can be triggered once every 0.8s.",
    },
    buffGenre: {
        Character: "Character BUFF",
        Weapon: "Weapon BUFF",
        Artifact: "Artifact BUFF",
        Resonance: "Resonance",
        Common: "Custom",
    }
}
