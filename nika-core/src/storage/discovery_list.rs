impl Storage {
    pub fn get_discovery_list(&self) -> Result<DailyList> {
        let items = self
            .db
            .prepare(
                "SELECT s.word_id, s.status, s.created_at, s.updated_at, d.created_at
                 FROM study_item as s
                 JOIN discovery_list as d ON s.word_id = d.word_id
                 ORDER BY d.created_at DESC",
            )?
            .query_map(params![], row_to_study_item)?
            .collect::<Result<Vec<StudyItem>, _>>()?;

        Ok(StudyList::new(items))
    }

    pub fn insert_discovery_item(&self, item: StudyItem) -> Result<()> {}
}
