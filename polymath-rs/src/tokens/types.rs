#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum TokenType {
    Greek(Greek),
    Operation(Operation),
    Misc(Misc),
    Relational(Relational),
    Arrow(Arrow),
    Logical(Logical),
    Function(Function),
    LBrace(LBrace),
    RBrace(RBrace),
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
    Division,
    Underscorce,
    Hat,
    Symbol,
    Number,
    Text,
    None,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Greek {
    Alpha,
    Beta,
    Gamma,
    UGamma,
    Delta,
    UDelta,
    Epsilon,
    VarEpsilon,
    Zeta,
    Eta,
    Theta,
    UTheta,
    VarTheta,
    Iota,
    Kappa,
    Lambda,
    ULambda,
    Mu,
    Nu,
    Xi,
    UXi,
    Pi,
    UPi,
    Rho,
    Sigma,
    USigma,
    Tau,
    Upsilon,
    Phi,
    UPhi,
    VarPhi,
    Chi,
    Psi,
    UPsi,
    Omega,
    UOmega,
}

pub const ALPHA: &str = "alpha";
pub const BETA: &str = "beta";
pub const GAMMA: &str = "gamma";
pub const U_GAMMA: &str = "Gamma";
pub const DELTA: &str = "delta";
pub const U_DELTA: &str = "Delta";
pub const EPSILON: &str = "epsilon";
pub const VAR_EPSILON: &str = "varepsilon";
pub const ZETA: &str = "zeta";
pub const ETA: &str = "eta";
pub const THETA: &str = "theta";
pub const U_THETA: &str = "Theta";
pub const VAR_THETA: &str = "vartheta";
pub const IOTA: &str = "iota";
pub const KAPPA: &str = "kappa";
pub const LAMBDA: &str = "lambda";
pub const U_LAMBDA: &str = "Lambda";
pub const MU: &str = "mu";
pub const NU: &str = "nu";
pub const XI: &str = "xi";
pub const U_XI: &str = "Xi";
pub const PI: &str = "pi";
pub const U_PI: &str = "Pi";
pub const RHO: &str = "rho";
pub const SIGMA: &str = "sigma";
pub const U_SIGMA: &str = "Sigma";
pub const TAU: &str = "tau";
pub const UPSILON: &str = "upsilon";
pub const PHI: &str = "phi";
pub const U_PHI: &str = "Phi";
pub const VAR_PHI: &str = "varphi";
pub const CHI: &str = "chi";
pub const PSI: &str = "psi";
pub const U_PSI: &str = "Psi";
pub const OMEGA: &str = "omega";
pub const U_OMEGA: &str = "Omega";

pub const GREEK: &[(&[&str], TokenType)] = &[
    (&[ALPHA], TokenType::Greek(Greek::Alpha)),
    (&[BETA], TokenType::Greek(Greek::Beta)),
    (&[GAMMA], TokenType::Greek(Greek::Gamma)),
    (&[U_GAMMA], TokenType::Greek(Greek::UGamma)),
    (&[DELTA], TokenType::Greek(Greek::Delta)),
    (&[U_DELTA], TokenType::Greek(Greek::UDelta)),
    (&[EPSILON], TokenType::Greek(Greek::Epsilon)),
    (&[VAR_EPSILON], TokenType::Greek(Greek::VarEpsilon)),
    (&[ZETA], TokenType::Greek(Greek::Zeta)),
    (&[ETA], TokenType::Greek(Greek::Eta)),
    (&[THETA], TokenType::Greek(Greek::Theta)),
    (&[U_THETA], TokenType::Greek(Greek::UTheta)),
    (&[VAR_THETA], TokenType::Greek(Greek::VarTheta)),
    (&[IOTA], TokenType::Greek(Greek::Iota)),
    (&[KAPPA], TokenType::Greek(Greek::Kappa)),
    (&[LAMBDA], TokenType::Greek(Greek::Lambda)),
    (&[U_LAMBDA], TokenType::Greek(Greek::ULambda)),
    (&[MU], TokenType::Greek(Greek::Mu)),
    (&[NU], TokenType::Greek(Greek::Nu)),
    (&[XI], TokenType::Greek(Greek::Xi)),
    (&[U_XI], TokenType::Greek(Greek::UXi)),
    (&[PI], TokenType::Greek(Greek::Pi)),
    (&[U_PI], TokenType::Greek(Greek::UPi)),
    (&[RHO], TokenType::Greek(Greek::Rho)),
    (&[SIGMA], TokenType::Greek(Greek::Sigma)),
    (&[U_SIGMA], TokenType::Greek(Greek::USigma)),
    (&[TAU], TokenType::Greek(Greek::Tau)),
    (&[UPSILON], TokenType::Greek(Greek::Upsilon)),
    (&[PHI], TokenType::Greek(Greek::Phi)),
    (&[U_PHI], TokenType::Greek(Greek::UPhi)),
    (&[VAR_PHI], TokenType::Greek(Greek::VarPhi)),
    (&[CHI], TokenType::Greek(Greek::Chi)),
    (&[PSI], TokenType::Greek(Greek::Psi)),
    (&[U_PSI], TokenType::Greek(Greek::UPsi)),
    (&[OMEGA], TokenType::Greek(Greek::Omega)),
    (&[U_OMEGA], TokenType::Greek(Greek::UOmega)),
];

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Misc {
    Int,
    OInt,
    Del,
    Grad,
    PlusMinus,
    EmptySet,
    Infinity,
    Aleph,
    Therefore,
    Because,
    LDots,
    CDots,
    VDots,
    DDots,
    DoublePipes,
    DoublePipesQuad,
    Angle,
    Frown,
    Triangle,
    Diamond,
    Square,
    LFloor,
    RFloor,
    LCeiling,
    RCeiling,
    Lim,
    CC,
    NN,
    QQ,
    RR,
    ZZ,
}

pub const INT: &str = "int";
pub const O_INT: &str = "oint";
pub const DEL: &str = "del";
pub const DEL_TEX: &str = "partial";
pub const GRAD: &str = "grad";
pub const GRAD_TEX: &str = "nabla";
pub const PM: &str = "+-";
pub const PM_TEX: &str = "pm";
pub const EMPTY_SET: &str = "O/";
pub const EMPTY_SET_TEX: &str = "emptyset";
pub const INFINITY: &str = "oo";
pub const INFINITY_TEX: &str = "infty";
pub const ALEPH: &str = "aleph";
pub const THEREFORE: &str = ":.";
pub const THEREFORE_TEX: &str = "therefore";
pub const BECAUSE: &str = ":'";
pub const BECAUSE_TEX: &str = "because";
pub const L_DOTS: &str = "|...|";
pub const L_DOTS_TEX: &str = "|ldots|";
pub const C_DOTS: &str = "|cdots|";
pub const V_DOTS: &str = "vdots";
pub const D_DOTS: &str = "ddots";
pub const DOUBLE_PIPES: &str = "|\\ |";
pub const DOUBLE_PIPES_QUAD: &str = "|quad|";
pub const ANGLE: &str = "/_";
pub const ANGLE_TEX: &str = "angle";
pub const FROWN: &str = "frown";
pub const TRIANGLE: &str = "/_\\";
pub const TRIANGLE_TEX: &str = "triangle";
pub const DIAMOND: &str = "diamond";
pub const SQUARE: &str = "square";
pub const L_FLOOR: &str = "|__";
pub const L_FLOOR_TEX: &str = "lfloor";
pub const R_FLOOR: &str = "__|";
pub const R_FLOOR_TEX: &str = "rfloor";
pub const L_CEILING: &str = "|~";
pub const L_CEILING_TEX: &str = "lceiling";
pub const R_CEILING: &str = "~|";
pub const R_CEILING_TEX: &str = "rceiling";
pub const LIM: &str = "lim";
pub const CC: &str = "CC";
pub const NN: &str = "NN";
pub const QQ: &str = "QQ";
pub const RR: &str = "RR";
pub const ZZ: &str = "ZZ";

pub const MISC: &[(&[&str], TokenType)] = &[
    (&[INT], TokenType::Misc(Misc::Int)),
    (&[O_INT], TokenType::Misc(Misc::OInt)),
    (&[DEL, DEL_TEX], TokenType::Misc(Misc::Del)),
    (&[GRAD, GRAD_TEX], TokenType::Misc(Misc::Grad)),
    (&[PM, PM_TEX], TokenType::Misc(Misc::PlusMinus)),
    (&[EMPTY_SET, EMPTY_SET_TEX], TokenType::Misc(Misc::EmptySet)),
    (&[INFINITY, INFINITY_TEX], TokenType::Misc(Misc::Infinity)),
    (&[ALEPH], TokenType::Misc(Misc::Aleph)),
    (
        &[THEREFORE, THEREFORE_TEX],
        TokenType::Misc(Misc::Therefore),
    ),
    (&[BECAUSE, BECAUSE_TEX], TokenType::Misc(Misc::Because)),
    (&[L_DOTS, L_DOTS_TEX], TokenType::Misc(Misc::LDots)),
    (&[C_DOTS], TokenType::Misc(Misc::CDots)),
    (&[V_DOTS], TokenType::Misc(Misc::VDots)),
    (&[D_DOTS], TokenType::Misc(Misc::DDots)),
    (&[DOUBLE_PIPES], TokenType::Misc(Misc::DoublePipes)),
    (&[DOUBLE_PIPES_QUAD], TokenType::Misc(Misc::DoublePipesQuad)),
    (&[ANGLE, ANGLE_TEX], TokenType::Misc(Misc::Angle)),
    (&[FROWN], TokenType::Misc(Misc::Frown)),
    (&[TRIANGLE, TRIANGLE_TEX], TokenType::Misc(Misc::Triangle)),
    (&[DIAMOND], TokenType::Misc(Misc::Diamond)),
    (&[SQUARE], TokenType::Misc(Misc::Square)),
    (&[L_FLOOR, L_FLOOR_TEX], TokenType::Misc(Misc::LFloor)),
    (&[R_FLOOR, R_FLOOR_TEX], TokenType::Misc(Misc::RFloor)),
    (&[L_CEILING, L_CEILING_TEX], TokenType::Misc(Misc::LCeiling)),
    (&[R_CEILING, R_CEILING_TEX], TokenType::Misc(Misc::RCeiling)),
    (&[LIM], TokenType::Misc(Misc::Lim)),
    (&[CC], TokenType::Misc(Misc::CC)),
    (&[NN], TokenType::Misc(Misc::NN)),
    (&[QQ], TokenType::Misc(Misc::QQ)),
    (&[RR], TokenType::Misc(Misc::RR)),
    (&[ZZ], TokenType::Misc(Misc::ZZ)),
];

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Relational {
    Equals,
    NotEquals,
    Lt,
    Gt,
    Lte,
    Gte,
    Mlt,
    Mgt,
    Prec,
    PrecEq,
    Succ,
    SuccEq,
    In,
    NotIn,
    Sub,
    Sup,
    SubEq,
    SupEq,
    Equiv,
    Cong,
    Approx,
    Prop,
}

pub const EQUALS: &str = "=";
pub const NOT_EQUALS: &str = "!=";
pub const NOT_EQUALS_TEX: &str = "ne";
pub const LT: &str = "<";
pub const LT_TEX: &str = "lt";
pub const GT: &str = ">";
pub const GT_TEX: &str = "gt";
pub const LTE: &str = "<=";
pub const LTE_TEX: &str = "le";
pub const GTE: &str = ">=";
pub const GTE_TEX: &str = "ge";
pub const MLT: &str = "mlt";
pub const MLT_TEX: &str = "ll";
pub const MGT: &str = "mgt";
pub const MGT_TEX: &str = "gg";
pub const PREC: &str = "-<";
pub const PREC_TEX: &str = "prec";
pub const PREC_EQ: &str = "-<=";
pub const PREC_EQ_TEX: &str = "preceq";
pub const SUCC: &str = ">-";
pub const SUCC_TEX: &str = "subb";
pub const SUCC_EQ: &str = ">-=";
pub const SUCC_EQ_TEX: &str = "succeq";
pub const IN: &str = "in";
pub const NOT_IN: &str = "!in";
pub const NOT_IN_TEX: &str = "notin";
pub const SUB: &str = "sub";
pub const SUB_TEX: &str = "subset";
pub const SUP: &str = "sup";
pub const SUP_TEX: &str = "supset";
pub const SUB_EQ: &str = "sube";
pub const SUB_EQ_TEX: &str = "subseteq";
pub const SUP_EQ: &str = "supe";
pub const SUP_EQ_TEX: &str = "supseteq";
pub const EQUIV: &str = "-=";
pub const EQUIV_TEX: &str = "equiv";
pub const CONG: &str = "~=";
pub const CONG_TEX: &str = "cong";
pub const APPROX: &str = "~~";
pub const APPROX_TEX: &str = "approx";
pub const PROP: &str = "prop";
pub const PROP_TEX: &str = "propto";

pub const RELATIONAL: &[(&[&str], TokenType)] = &[
    (&[EQUALS], TokenType::Relational(Relational::Equals)),
    (
        &[NOT_EQUALS, NOT_EQUALS_TEX],
        TokenType::Relational(Relational::NotEquals),
    ),
    (&[LT, LT_TEX], TokenType::Relational(Relational::Lt)),
    (&[GT, GT_TEX], TokenType::Relational(Relational::Gt)),
    (&[LTE, LTE_TEX], TokenType::Relational(Relational::Lte)),
    (&[GTE, GTE_TEX], TokenType::Relational(Relational::Gte)),
    (&[MLT, MLT_TEX], TokenType::Relational(Relational::Mlt)),
    (&[MGT, MGT_TEX], TokenType::Relational(Relational::Mgt)),
    (&[PREC, PREC_TEX], TokenType::Relational(Relational::Prec)),
    (
        &[PREC_EQ, PREC_EQ_TEX],
        TokenType::Relational(Relational::PrecEq),
    ),
    (&[SUCC, SUCC_TEX], TokenType::Relational(Relational::Succ)),
    (
        &[SUCC_EQ, SUCC_EQ_TEX],
        TokenType::Relational(Relational::SuccEq),
    ),
    (&[IN], TokenType::Relational(Relational::NotIn)),
    (
        &[NOT_IN, NOT_IN_TEX],
        TokenType::Relational(Relational::NotIn),
    ),
    (&[SUB, SUB_TEX], TokenType::Relational(Relational::Sub)),
    (&[SUP, SUP_TEX], TokenType::Relational(Relational::Sup)),
    (
        &[SUB_EQ, SUB_EQ_TEX],
        TokenType::Relational(Relational::SubEq),
    ),
    (
        &[SUP_EQ, SUP_EQ_TEX],
        TokenType::Relational(Relational::SupEq),
    ),
    (
        &[EQUIV, EQUIV_TEX],
        TokenType::Relational(Relational::Equiv),
    ),
    (&[CONG, CONG_TEX], TokenType::Relational(Relational::Cong)),
    (
        &[APPROX, APPROX_TEX],
        TokenType::Relational(Relational::Approx),
    ),
    (&[PROP, PROP_TEX], TokenType::Relational(Relational::Prop)),
];

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Arrow {
    UpArrow,
    DownArrow,
    RightArrow,
    ToArrow,
    RightArrowTail,
    RightArrowTwoHead,
    RightArrowTwoHeadTail,
    MapsTo,
    LeftArrow,
    LeftRightArrow,
    DoubleRightArrow,
    DoubleLeftArrow,
    DoubleLeftRightArrow,
}

pub const UP_ARROW: &str = "uarr";
pub const UP_ARROW_TEX: &str = "uparrow";
pub const DOWN_ARROW: &str = "darr";
pub const DOWN_ARROW_TEX: &str = "downarrow";
pub const RIGHT_ARROW: &str = "rarr";
pub const RIGHT_ARROW_TEX: &str = "rightarrow";
pub const TO_ARROW: &str = "->";
pub const TO_ARROW_TEX: &str = "to";
pub const R_ARROW_TAIL: &str = ">->";
pub const R_ARROW_TAIL_TEX: &str = "rightarrowtail";
pub const R_ARROW_TWO_HEAD: &str = "->>";
pub const R_ARROW_TWO_HEAD_TEX: &str = "twoheadrightarrow";
pub const R_ARROW_TWO_HEAD_TAIL: &str = ">->>";
pub const R_ARROW_TWO_HEAD_TAIL_TEX: &str = "twoheadrightarrowtail";
pub const MAPS_TO: &str = "|->";
pub const MAPS_TO_TEX: &str = "mapsto";
pub const LEFT_ARROW: &str = "larr";
pub const LEFT_ARROW_TEX: &str = "leftarrow";
pub const LEFT_RIGHT_ARROW: &str = "harr";
pub const LEFT_RIGHT_ARROW_TEX: &str = "leftrightarrow";
pub const DOUBLE_RIGHT_ARROW: &str = "rArr";
pub const DOUBLE_RIGHT_ARROW_TEX: &str = "Rightarrow";
pub const DOUBLE_LEFT_ARROW: &str = "lArr";
pub const DOUBLE_LEFT_ARROW_TEX: &str = "Leftarrow";
pub const DOUBLE_LEFT_RIGHT_ARROW: &str = "hArr";
pub const DOUBLE_LEFT_RIGHT_ARROW_TEX: &str = "Leftrightarrow";

pub const ARROWS: &[(&[&str], TokenType)] = &[
    (&[UP_ARROW, UP_ARROW_TEX], TokenType::Arrow(Arrow::UpArrow)),
    (
        &[DOWN_ARROW, DOWN_ARROW_TEX],
        TokenType::Arrow(Arrow::DownArrow),
    ),
    (
        &[RIGHT_ARROW, RIGHT_ARROW_TEX],
        TokenType::Arrow(Arrow::RightArrow),
    ),
    (&[TO_ARROW, TO_ARROW_TEX], TokenType::Arrow(Arrow::ToArrow)),
    (
        &[R_ARROW_TAIL, R_ARROW_TAIL_TEX],
        TokenType::Arrow(Arrow::RightArrowTail),
    ),
    (
        &[R_ARROW_TWO_HEAD, R_ARROW_TWO_HEAD_TEX],
        TokenType::Arrow(Arrow::RightArrowTwoHead),
    ),
    (
        &[R_ARROW_TWO_HEAD_TAIL, R_ARROW_TWO_HEAD_TAIL_TEX],
        TokenType::Arrow(Arrow::RightArrowTwoHeadTail),
    ),
    (&[MAPS_TO, MAPS_TO_TEX], TokenType::Arrow(Arrow::MapsTo)),
    (
        &[LEFT_ARROW, LEFT_ARROW_TEX],
        TokenType::Arrow(Arrow::LeftArrow),
    ),
    (
        &[LEFT_RIGHT_ARROW, LEFT_RIGHT_ARROW_TEX],
        TokenType::Arrow(Arrow::LeftRightArrow),
    ),
    (
        &[DOUBLE_RIGHT_ARROW, DOUBLE_RIGHT_ARROW_TEX],
        TokenType::Arrow(Arrow::DoubleRightArrow),
    ),
    (
        &[DOUBLE_LEFT_ARROW, DOUBLE_LEFT_ARROW_TEX],
        TokenType::Arrow(Arrow::DoubleLeftArrow),
    ),
    (
        &[DOUBLE_LEFT_RIGHT_ARROW, DOUBLE_LEFT_RIGHT_ARROW_TEX],
        TokenType::Arrow(Arrow::DoubleLeftRightArrow),
    ),
];

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Logical {
    And,
    Or,
    Not,
    Implies,
    If,
    Iff,
    ForAll,
    Exists,
    Bot,
    Top,
    VDash,
    Models,
}

pub const AND: &str = "and";
pub const OR: &str = "or";
pub const NOT: &str = "not";
pub const NOT_TEX: &str = "neq";
pub const IMPLIES: &str = "=>";
pub const IMPLIES_TEX: &str = "implies";
pub const IF: &str = "if";
pub const IFF: &str = "<=>";
pub const IFF_TEX: &str = "iff";
pub const FOR_ALL: &str = "AA";
pub const FOR_ALL_TEX: &str = "forall";
pub const EXISTS: &str = "EE";
pub const EXISTS_TEX: &str = "exists";
pub const BOT: &str = "_|_";
pub const BOT_TEX: &str = "bot";
pub const TOP: &str = "TT";
pub const TOP_TEX: &str = "top";
pub const V_DASH: &str = "|--";
pub const V_DASH_TEX: &str = "vdash";
pub const MODELS: &str = "|==";
pub const MODELS_TEX: &str = "models";

pub const LOGICAL: &[(&[&str], TokenType)] = &[
    (&[AND], TokenType::Logical(Logical::And)),
    (&[OR], TokenType::Logical(Logical::Or)),
    (&[NOT, NOT_TEX], TokenType::Logical(Logical::Not)),
    (
        &[IMPLIES, IMPLIES_TEX],
        TokenType::Logical(Logical::Implies),
    ),
    (&[IF], TokenType::Logical(Logical::If)),
    (&[IFF, IFF_TEX], TokenType::Logical(Logical::Iff)),
    (&[FOR_ALL, FOR_ALL_TEX], TokenType::Logical(Logical::ForAll)),
    (&[EXISTS, EXISTS_TEX], TokenType::Logical(Logical::Exists)),
    (&[BOT, BOT_TEX], TokenType::Logical(Logical::Bot)),
    (&[TOP, TOP_TEX], TokenType::Logical(Logical::Top)),
    (&[V_DASH, V_DASH_TEX], TokenType::Logical(Logical::VDash)),
    (&[MODELS, MODELS_TEX], TokenType::Logical(Logical::Models)),
];

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    CDot,
    Ast,
    Star,
    Slash,
    Backslash,
    Times,
    Div,
    LTimes,
    RTimes,
    Bowtie,
    Circ,
    OPlus,
    OTimes,
    ODot,
    Sum,
    Prod,
    Wedge,
    BigWedge,
    Vee,
    BigVee,
    Cap,
    BigCap,
    Cup,
    BigCup,
}

pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const C_DOT: &str = "*";
pub const C_DOT_TEX: &str = "cdot";
pub const AST: &str = "**";
pub const AST_TEX: &str = "ast";
pub const STAR: &str = "***";
pub const STAR_TEX: &str = "star";
pub const SLASH: &str = "//";
pub const BACKSLASH: &str = "\\\\";
pub const BACKSLASH_TEX_1: &str = "backslash";
pub const BACKSLASH_TEX_2: &str = "setminus";
pub const TIMES: &str = "xx";
pub const TIMES_TEX: &str = "times";
pub const DIV: &str = "-:";
pub const DIV_TEX: &str = "div";
pub const L_TIMES: &str = "|><";
pub const L_TIMES_TEX: &str = "ltimes";
pub const R_TIMES: &str = "><|";
pub const R_TIMES_TEX: &str = "rtimes";
pub const BOWTIE: &str = "|><|";
pub const BOWTIE_TEX: &str = "bowtie";
pub const CIRC: &str = "@";
pub const CIRC_TEX: &str = "circ";
pub const O_PLUS: &str = "o+";
pub const O_PLUS_TEX: &str = "oplus";
pub const O_TIMES: &str = "ox";
pub const O_TIMES_TEX: &str = "otimes";
pub const O_DOT: &str = "o.";
pub const O_DOT_TEX: &str = "odot";
pub const SUM: &str = "sum";
pub const PROD: &str = "prod";
pub const WEDGE: &str = "^^";
pub const WEDGE_TEX: &str = "wedge";
pub const BIG_WEDGE: &str = "^^^";
pub const BIG_WEDGE_TEX: &str = "bigwedge";
pub const VEE: &str = "vv";
pub const VEE_TEX: &str = "vee";
pub const BIG_VEE: &str = "vvv";
pub const BIG_VEE_TEX: &str = "bigvee";
pub const CAP: &str = "nn";
pub const CAP_TEX: &str = "cap";
pub const BIG_CAP: &str = "nnn";
pub const BIG_CAP_TEX: &str = "bigcap";
pub const CUP: &str = "uu";
pub const CUP_TEX: &str = "cup";
pub const BIG_CUP: &str = "uuu";
pub const BIG_CUP_TEX: &str = "bigcup";

pub const OPERATION: &[(&[&str], TokenType)] = &[
    (&[C_DOT, C_DOT_TEX], TokenType::Operation(Operation::CDot)),
    (&[AST, AST_TEX], TokenType::Operation(Operation::Ast)),
    (&[STAR, STAR_TEX], TokenType::Operation(Operation::Star)),
    (&[SLASH], TokenType::Operation(Operation::Slash)),
    (
        &[BACKSLASH, BACKSLASH_TEX_1, BACKSLASH_TEX_2],
        TokenType::Operation(Operation::Backslash),
    ),
    (&[TIMES, TIMES_TEX], TokenType::Operation(Operation::Times)),
    (&[DIV, DIV_TEX], TokenType::Operation(Operation::Div)),
    (
        &[L_TIMES, L_TIMES_TEX],
        TokenType::Operation(Operation::LTimes),
    ),
    (
        &[R_TIMES, R_TIMES_TEX],
        TokenType::Operation(Operation::RTimes),
    ),
    (
        &[BOWTIE, BOWTIE_TEX],
        TokenType::Operation(Operation::Bowtie),
    ),
    (&[CIRC, CIRC_TEX], TokenType::Operation(Operation::Circ)),
    (
        &[O_PLUS, O_PLUS_TEX],
        TokenType::Operation(Operation::OPlus),
    ),
    (
        &[O_TIMES, O_TIMES_TEX],
        TokenType::Operation(Operation::Times),
    ),
    (&[O_DOT], TokenType::Operation(Operation::ODot)),
    (&[SUM], TokenType::Operation(Operation::Sum)),
    (&[PROD], TokenType::Operation(Operation::Prod)),
    (
        &[BIG_WEDGE, BIG_WEDGE_TEX],
        TokenType::Operation(Operation::BigWedge),
    ),
    (&[WEDGE, WEDGE_TEX], TokenType::Operation(Operation::Wedge)),
    (&[VEE, VEE_TEX], TokenType::Operation(Operation::Vee)),
    (
        &[BIG_CAP, BIG_CAP_TEX],
        TokenType::Operation(Operation::BigCap),
    ),
    (&[CAP, CAP_TEX], TokenType::Operation(Operation::Cap)),
    (
        &[BIG_CUP, BIG_CUP_TEX],
        TokenType::Operation(Operation::BigCup),
    ),
    (&[CUP, CUP_TEX], TokenType::Operation(Operation::Cup)),
    (&[PLUS], TokenType::Operation(Operation::Plus)),
    (&[MINUS], TokenType::Operation(Operation::Minus)),
];

// Braces

pub const L_PAREN: &str = "(";
pub const L_BRACKET: &str = "[";
pub const L_BRACE: &str = "{";
pub const L_COLON_BRACE: &str = "{:";
pub const L_ANGLE: &str = "<<";

pub const L_ANGLE_TEX: &str = "langle";

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum LBrace {
    LParen,
    LBracket,
    LBrace,
    LColonBrace,
    LAngle,
}

pub const LBRACES: &[(&[&str], TokenType)] = &[
    (&[L_COLON_BRACE], TokenType::LBrace(LBrace::LColonBrace)),
    (&[L_PAREN], TokenType::LBrace(LBrace::LParen)),
    (&[L_BRACKET], TokenType::LBrace(LBrace::LBracket)),
    (&[L_BRACE], TokenType::LBrace(LBrace::LBrace)),
    (&[L_ANGLE, L_ANGLE_TEX], TokenType::LBrace(LBrace::LAngle)),
];

pub const R_PAREN: &str = ")";
pub const R_BRACKET: &str = "]";
pub const R_BRACE: &str = "}";
pub const R_COLON_BRACE: &str = ":}";
pub const R_ANGLE: &str = ">>";
pub const R_ANGLE_TEX: &str = "rangle";

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum RBrace {
    RParen,
    RBracket,
    RBrace,
    RColonBrace,
    RAngle,
}

pub const RBRACES: &[(&[&str], TokenType)] = &[
    (&[R_COLON_BRACE], TokenType::RBrace(RBrace::RColonBrace)),
    (&[R_PAREN], TokenType::RBrace(RBrace::RParen)),
    (&[R_BRACKET], TokenType::RBrace(RBrace::RBracket)),
    (&[R_BRACE], TokenType::RBrace(RBrace::RBrace)),
    (&[R_ANGLE, R_ANGLE_TEX], TokenType::RBrace(RBrace::RAngle)),
];

// Unary operators

pub const HAT: &str = "hat";
pub const BAR: &str = "bar";
pub const BAR_TEX: &str = "overline";
pub const UL: &str = "ul";
pub const UL_TEX: &str = "underline";
pub const VEC: &str = "vec";
pub const TILDE: &str = "tilde";
pub const DOT: &str = "dot";
pub const D_DOT: &str = "ddot";
pub const U_BRACE: &str = "ubrace";
pub const U_BRACE_TEX: &str = "underbrace";
pub const O_BRACE: &str = "obrace";
pub const O_BRACE_TEX: &str = "overbrace";
pub const CANCEL: &str = "cancel";

pub const SQRT: &str = "sqrt";
pub const TEXT: &str = "text";
pub const ABS: &str = "abs";
pub const FLOOR: &str = "floor";
pub const CEIL: &str = "ceil";
pub const NORM: &str = "norm";

pub const UNARY_OPERATORS: &[(&[&str], TokenType)] = &[
    (&[HAT], TokenType::UnaryOperator(UnaryOperator::Hat)),
    (
        &[BAR, BAR_TEX],
        TokenType::UnaryOperator(UnaryOperator::Bar),
    ),
    (&[UL, UL_TEX], TokenType::UnaryOperator(UnaryOperator::Ul)),
    (&[VEC], TokenType::UnaryOperator(UnaryOperator::Vec)),
    (&[TILDE], TokenType::UnaryOperator(UnaryOperator::Tilde)),
    (&[DOT], TokenType::UnaryOperator(UnaryOperator::Dot)),
    (&[D_DOT], TokenType::UnaryOperator(UnaryOperator::DDot)),
    (
        &[U_BRACE, U_BRACE_TEX],
        TokenType::UnaryOperator(UnaryOperator::UBrace),
    ),
    (
        &[O_BRACE, O_BRACE_TEX],
        TokenType::UnaryOperator(UnaryOperator::OBrace),
    ),
    (&[CANCEL], TokenType::UnaryOperator(UnaryOperator::Cancel)),
    (&[SQRT], TokenType::UnaryOperator(UnaryOperator::Sqrt)),
    (&[TEXT], TokenType::UnaryOperator(UnaryOperator::Text)),
    (&[ABS], TokenType::UnaryOperator(UnaryOperator::Abs)),
    (&[FLOOR], TokenType::UnaryOperator(UnaryOperator::Floor)),
    (&[CEIL], TokenType::UnaryOperator(UnaryOperator::Ceil)),
    (&[NORM], TokenType::UnaryOperator(UnaryOperator::Norm)),
];

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum UnaryOperator {
    Hat,
    Bar,
    Ul,
    Vec,
    Tilde,
    Dot,
    DDot,
    UBrace,
    OBrace,
    Cancel,
    Sqrt,
    Text,
    Abs,
    Floor,
    Ceil,
    Norm,
}

// Binary operators

pub const ROOT: &str = "root";
pub const OVERSET: &str = "overset";
pub const UNDERSET: &str = "underset";
pub const COLOR: &str = "color";

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum BinaryOperator {
    Root,
    Overset,
    Underset,
    Color,
}

pub const BINARY_OPERATORS: &[(&[&str], TokenType)] = &[
    (&[ROOT], TokenType::BinaryOperator(BinaryOperator::Root)),
    (
        &[OVERSET],
        TokenType::BinaryOperator(BinaryOperator::Overset),
    ),
    (
        &[UNDERSET],
        TokenType::BinaryOperator(BinaryOperator::Underset),
    ),
    (&[COLOR], TokenType::BinaryOperator(BinaryOperator::Color)),
];

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Sec,
    Csc,
    Cot,
    Arcsin,
    Arccos,
    Arctan,
    Sinh,
    Cosh,
    Tanh,
    Sech,
    Csch,
    Coth,
    Exp,
    Log,
    Ln,
    Det,
    Dim,
    Mod,
    Gcd,
    Lcm,
    Lub,
    Glb,
    Min,
    Max,
    F,
    G,
}

pub const F_SIN: &str = "sin";
pub const F_COS: &str = "cos";
pub const F_TAN: &str = "tan";
pub const F_SEC: &str = "sec";
pub const F_CSC: &str = "csc";
pub const F_COT: &str = "cot";
pub const F_ARCSIN: &str = "arcsin";
pub const F_ARCCOS: &str = "arccos";
pub const F_ARCTAN: &str = "arctan";
pub const F_SINH: &str = "sinh";
pub const F_COSH: &str = "cosh";
pub const F_TANH: &str = "tanh";
pub const F_SECH: &str = "sech";
pub const F_CSCH: &str = "csch";
pub const F_COTH: &str = "coth";
pub const F_EXP: &str = "exp";
pub const F_LOG: &str = "log";
pub const F_LN: &str = "ln";
pub const F_DET: &str = "det";
pub const F_DIM: &str = "dim";
pub const F_MOD: &str = "mod";
pub const F_GCD: &str = "gcd";
pub const F_LCM: &str = "lcm";
pub const F_LUB: &str = "lub";
pub const F_GLB: &str = "glb";
pub const F_MIN: &str = "min";
pub const F_MAX: &str = "max";
pub const F_F: &str = "f";
pub const F_G: &str = "g";

pub const FUNCTION: &[(&[&str], TokenType)] = &[
    (&[F_SIN], TokenType::Function(Function::Sin)),
    (&[F_COS], TokenType::Function(Function::Cos)),
    (&[F_TAN], TokenType::Function(Function::Tan)),
    (&[F_SEC], TokenType::Function(Function::Sec)),
    (&[F_CSC], TokenType::Function(Function::Csc)),
    (&[F_COT], TokenType::Function(Function::Cot)),
    (&[F_ARCSIN], TokenType::Function(Function::Arcsin)),
    (&[F_ARCCOS], TokenType::Function(Function::Arccos)),
    (&[F_ARCTAN], TokenType::Function(Function::Arctan)),
    (&[F_SINH], TokenType::Function(Function::Sinh)),
    (&[F_COSH], TokenType::Function(Function::Cosh)),
    (&[F_TANH], TokenType::Function(Function::Tanh)),
    (&[F_SECH], TokenType::Function(Function::Sech)),
    (&[F_CSCH], TokenType::Function(Function::Csch)),
    (&[F_COTH], TokenType::Function(Function::Coth)),
    (&[F_EXP], TokenType::Function(Function::Exp)),
    (&[F_LOG], TokenType::Function(Function::Log)),
    (&[F_LN], TokenType::Function(Function::Ln)),
    (&[F_DET], TokenType::Function(Function::Det)),
    (&[F_DIM], TokenType::Function(Function::Dim)),
    (&[F_MOD], TokenType::Function(Function::Mod)),
    (&[F_GCD], TokenType::Function(Function::Gcd)),
    (&[F_LCM], TokenType::Function(Function::Lcm)),
    (&[F_LUB], TokenType::Function(Function::Lub)),
    (&[F_GLB], TokenType::Function(Function::Glb)),
    (&[F_MIN], TokenType::Function(Function::Min)),
    (&[F_MAX], TokenType::Function(Function::Max)),
    (&[F_F], TokenType::Function(Function::F)),
    (&[F_G], TokenType::Function(Function::G)),
];
