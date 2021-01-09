#[macro_export]
macro_rules! edge_points_to_check {
    ($nanobot:expr, $region:expr, 0) => {
        vec![
            ($nanobot.position.0, $region.from.1, $region.from.2),
            ($nanobot.position.0, $region.from.1, $region.to.2),
            ($nanobot.position.0, $region.to.1, $region.from.2),
            ($nanobot.position.0, $region.to.1, $region.to.2),
        ]
    };
    ($nanobot:expr, $region:expr, 1) => {
        vec![
            ($region.from.0, $nanobot.position.1, $region.from.2),
            ($region.from.0, $nanobot.position.1, $region.to.2),
            ($region.to.0, $nanobot.position.1, $region.from.2),
            ($region.to.0, $nanobot.position.1, $region.to.2),
        ]
    };
    ($nanobot:expr, $region:expr, 2) => {
        vec![
            ($region.from.0, $region.from.1, $nanobot.position.2),
            ($region.from.0, $region.to.1, $nanobot.position.2),
            ($region.to.0, $region.from.1, $nanobot.position.2),
            ($region.to.0, $region.to.1, $nanobot.position.2),
        ]
    };
}

#[macro_export]
macro_rules! check_if_edge_overlaps {
    ($nanobot:expr, $region:expr, $dim:tt) => {
        if ($region.from.$dim..=$region.to.$dim).contains(&$nanobot.position.$dim) {
            let points = $crate::edge_points_to_check![$nanobot, $region, $dim];
            points.iter().any(|&p| $nanobot.in_range(p))
        } else {
            false
        }
    };
}
