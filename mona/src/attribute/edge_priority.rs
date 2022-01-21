pub enum EdgePriority {
    // atk_base -> atk_percentage
    PriorityBase,

    // hp_percentage, hp_fixed, hp_base -> hp
    PriorityHP,
    // def_percentage, def_fixed, def_base -> def
    PriorityDEF,

    PriorityHP2ATK,
    // atk_percentage, atk_fixed, atk_base -> atk
    PriorityATK,



    // recharge -> bonus
    // atk -> other
    PRIORITY300,
}