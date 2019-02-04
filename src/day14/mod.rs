struct RecipesContainer {
    recipes: Vec<u8>,
    elf1: usize,
    elf2: usize,
}

impl RecipesContainer {
    fn new(start: Vec<u8>) -> Result<RecipesContainer, ()> {
        if start.len() < 2 {
            Err(())
        } else {
            Ok(RecipesContainer { recipes: start, elf1: 0, elf2: 1 })
        }
    }

    fn tick(&mut self) -> (u8, Option<u8>) {
        let new_recipe_num = self.recipes[self.elf1] + self.recipes[self.elf2];

        let ret_value = {
            if new_recipe_num > 9 {
                self.recipes.push(new_recipe_num / 10);
                self.recipes.push(new_recipe_num % 10);
                (new_recipe_num / 10, Some(new_recipe_num % 10))
            } else {
                self.recipes.push(new_recipe_num);
                (new_recipe_num, None)
            }
        };

        self.elf1 = (self.elf1 + ((self.recipes[self.elf1] as usize) + 1)) % self.recipes.len();
        self.elf2 = (self.elf2 + ((self.recipes[self.elf2] as usize) + 1)) % self.recipes.len();
        ret_value
    }

    fn ten_after_n(&mut self, n: usize) -> String {
        while self.recipes.len() < n + 10 {
            self.tick();
        }

        let ten_recipes_after = self.recipes[n..(n + 10)]
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("");

        ten_recipes_after
    }

    fn until_matches(&mut self, desired: &[u8]) -> usize {
        while self.recipes.len() < desired.len() + 1 {
            self.tick();
        }

        loop {
            if desired == &self.recipes[(self.recipes.len() - desired.len())..self.recipes.len()] {
                break self.recipes.len() - desired.len();
            }
            if desired == &self.recipes[(self.recipes.len() - desired.len() - 1)..(self.recipes.len() - 1)] {
                break self.recipes.len() - desired.len() - 1;
            }
            self.tick();
        }
    }
}

pub fn day14() {
    let mut recipes = vec![3u8, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;

    let from_recipe = 540561;

    let mut recipes = RecipesContainer::new(vec![3, 7]).unwrap();

    println!("ten_after_{}: {}", 540561, recipes.ten_after_n(540561));
    let desired= vec![5, 4, 0, 5, 6, 1];
    let mut recipes = RecipesContainer::new(vec![3, 7]).unwrap();
    println!("before {:?}: {}", desired, recipes.until_matches(&desired));
}
