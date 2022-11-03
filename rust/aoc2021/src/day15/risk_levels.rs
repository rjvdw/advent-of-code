use std::io;

use grid::Grid;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::grid::iterators::WithGridIterator;
use rdcl_aoc_helpers::grid::neighbours::WithNeighbours;
use rdcl_aoc_helpers::math::taxi_cab_2d;
use rdcl_aoc_helpers::search::Navigable;

use shared::numeric_grid;

pub struct RiskLevels {
    map: Grid<u8>,
}

impl RiskLevels {
    pub fn find_optimal_path(&self) -> Option<u64> {
        let start = (0, 0);
        let end = (self.map.rows() - 1, self.map.cols() - 1);
        let path = self.find_shortest_path(&start, &end)?;

        Some(
            path.iter()
                .copied()
                .filter(|&point| point != start)
                .map(|(row, col)| self.map[row][col] as u64)
                .sum(),
        )
    }

    pub fn transform(&self) -> RiskLevels {
        let (rows, cols) = self.map.size();
        let mut map = Grid::new(rows * 5, cols * 5);
        for y in 0..5 {
            for x in 0..5 {
                for (row, col) in self.map.iter_row_col() {
                    let row_t = row + rows * (y as usize);
                    let col_t = col + cols * (x as usize);
                    let value = (self.map[row][col] + x + y - 1) % 9 + 1;
                    map[row_t][col_t] = value;
                }
            }
        }
        RiskLevels { map }
    }

    pub fn parse<I>(input: I) -> Result<RiskLevels, ParseError>
    where
        I: Iterator<Item = io::Result<String>>,
    {
        numeric_grid::parse(input).map(|map| RiskLevels { map })
    }
}

impl Navigable for RiskLevels {
    type Point = (usize, usize);

    fn distance_score(&self, a: &Self::Point, b: &Self::Point) -> u64 {
        taxi_cab_2d(*a, *b) as u64
    }

    fn get_neighbours(&self, point: &Self::Point) -> Vec<(u64, Self::Point)> {
        self.map
            .neighbours(*point, false)
            .iter()
            .map(|&(row, col)| (self.map[row][col] as u64, (row, col)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimal_path_1() {
        assert_eq!(risk_levels().find_optimal_path(), Some(40));
    }

    #[test]
    fn test_optimal_path_2() {
        let risk_levels = risk_levels().transform();
        assert_eq!(risk_levels.map, transformed());
        assert_eq!(risk_levels.find_optimal_path(), Some(315));
    }

    fn risk_levels() -> RiskLevels {
        let input = vec![
            Ok("1163751742".to_string()),
            Ok("1381373672".to_string()),
            Ok("2136511328".to_string()),
            Ok("3694931569".to_string()),
            Ok("7463417111".to_string()),
            Ok("1319128137".to_string()),
            Ok("1359912421".to_string()),
            Ok("3125421639".to_string()),
            Ok("1293138521".to_string()),
            Ok("2311944581".to_string()),
        ];
        RiskLevels::parse(input.into_iter()).unwrap()
    }

    fn transformed() -> Grid<u8> {
        let lines = vec![
            Ok("11637517422274862853338597396444961841755517295286".to_string()),
            Ok("13813736722492484783351359589446246169155735727126".to_string()),
            Ok("21365113283247622439435873354154698446526571955763".to_string()),
            Ok("36949315694715142671582625378269373648937148475914".to_string()),
            Ok("74634171118574528222968563933317967414442817852555".to_string()),
            Ok("13191281372421239248353234135946434524615754563572".to_string()),
            Ok("13599124212461123532357223464346833457545794456865".to_string()),
            Ok("31254216394236532741534764385264587549637569865174".to_string()),
            Ok("12931385212314249632342535174345364628545647573965".to_string()),
            Ok("23119445813422155692453326671356443778246755488935".to_string()),
            Ok("22748628533385973964449618417555172952866628316397".to_string()),
            Ok("24924847833513595894462461691557357271266846838237".to_string()),
            Ok("32476224394358733541546984465265719557637682166874".to_string()),
            Ok("47151426715826253782693736489371484759148259586125".to_string()),
            Ok("85745282229685639333179674144428178525553928963666".to_string()),
            Ok("24212392483532341359464345246157545635726865674683".to_string()),
            Ok("24611235323572234643468334575457944568656815567976".to_string()),
            Ok("42365327415347643852645875496375698651748671976285".to_string()),
            Ok("23142496323425351743453646285456475739656758684176".to_string()),
            Ok("34221556924533266713564437782467554889357866599146".to_string()),
            Ok("33859739644496184175551729528666283163977739427418".to_string()),
            Ok("35135958944624616915573572712668468382377957949348".to_string()),
            Ok("43587335415469844652657195576376821668748793277985".to_string()),
            Ok("58262537826937364893714847591482595861259361697236".to_string()),
            Ok("96856393331796741444281785255539289636664139174777".to_string()),
            Ok("35323413594643452461575456357268656746837976785794".to_string()),
            Ok("35722346434683345754579445686568155679767926678187".to_string()),
            Ok("53476438526458754963756986517486719762859782187396".to_string()),
            Ok("34253517434536462854564757396567586841767869795287".to_string()),
            Ok("45332667135644377824675548893578665991468977611257".to_string()),
            Ok("44961841755517295286662831639777394274188841538529".to_string()),
            Ok("46246169155735727126684683823779579493488168151459".to_string()),
            Ok("54698446526571955763768216687487932779859814388196".to_string()),
            Ok("69373648937148475914825958612593616972361472718347".to_string()),
            Ok("17967414442817852555392896366641391747775241285888".to_string()),
            Ok("46434524615754563572686567468379767857948187896815".to_string()),
            Ok("46833457545794456865681556797679266781878137789298".to_string()),
            Ok("64587549637569865174867197628597821873961893298417".to_string()),
            Ok("45364628545647573965675868417678697952878971816398".to_string()),
            Ok("56443778246755488935786659914689776112579188722368".to_string()),
            Ok("55172952866628316397773942741888415385299952649631".to_string()),
            Ok("57357271266846838237795794934881681514599279262561".to_string()),
            Ok("65719557637682166874879327798598143881961925499217".to_string()),
            Ok("71484759148259586125936169723614727183472583829458".to_string()),
            Ok("28178525553928963666413917477752412858886352396999".to_string()),
            Ok("57545635726865674683797678579481878968159298917926".to_string()),
            Ok("57944568656815567976792667818781377892989248891319".to_string()),
            Ok("75698651748671976285978218739618932984172914319528".to_string()),
            Ok("56475739656758684176786979528789718163989182927419".to_string()),
            Ok("67554889357866599146897761125791887223681299833479".to_string()),
        ];
        numeric_grid::parse(lines.into_iter()).unwrap()
    }
}
