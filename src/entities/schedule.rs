use super::Conference;

pub struct Schedule {
    pub name: String,
    schedule: [Vec<Conference>; 7],
}

impl Schedule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            schedule: [const { Vec::new() }; 7],
        }
    }

    pub fn get_day(&self, day: usize) -> &Vec<Conference> {
        assert!(day <= 7 && day > 0);
        &self.schedule[day - 1]
    }

    pub fn update_conference(
        &mut self,
        day: usize,
        index: usize,
        conference: Conference,
    ) -> Result<(), String> {
        assert!(day <= 7 && day > 0);

        *self.schedule[day - 1]
            .get_mut(index)
            .ok_or("Conference not found")? = conference;

        self.sort_conferences();
        Ok(())
    }

    pub fn add_conference(&mut self, day: usize, conference: Conference) {
        assert!(day <= 7 && day > 0);
        self.schedule[day - 1].push(conference);
        self.sort_conferences();
    }

    pub fn remove_conference(&mut self, day: usize, index: usize) {
        assert!(day <= 7 && day > 0);
        self.schedule[day - 1].remove(index);
    }

    fn sort_conferences(&mut self) {
        for day in self.schedule.iter_mut() {
            day.sort_by(|a, b| a.start_time.cmp(&b.start_time));
        }
    }

    pub fn get_conference_count_by_day(&self) -> Vec<usize> {
        self.schedule.iter().map(|day| day.len()).collect()
    }
}
