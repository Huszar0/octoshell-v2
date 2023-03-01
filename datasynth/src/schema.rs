// @generated automatically by Diesel CLI.

diesel::table! {
    active_storage_attachments (id) {
        id -> Int8,
        name -> Varchar,
        record_type -> Varchar,
        record_id -> Int8,
        blob_id -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    active_storage_blobs (id) {
        id -> Int8,
        key -> Varchar,
        filename -> Varchar,
        content_type -> Nullable<Varchar>,
        metadata -> Nullable<Text>,
        byte_size -> Int8,
        checksum -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    announcement_recipients (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        announcement_id -> Nullable<Int4>,
    }
}

diesel::table! {
    announcements (id) {
        id -> Int4,
        title_ru -> Nullable<Varchar>,
        reply_to -> Nullable<Varchar>,
        body_ru -> Nullable<Text>,
        attachment -> Nullable<Varchar>,
        is_special -> Nullable<Bool>,
        state -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        created_by_id -> Nullable<Int4>,
        title_en -> Nullable<Varchar>,
        body_en -> Nullable<Text>,
    }
}

diesel::table! {
    api_access_keys (id) {
        id -> Int4,
        key -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    api_access_keys_exports (id) {
        id -> Int4,
        access_key_id -> Nullable<Int4>,
        export_id -> Nullable<Int4>,
    }
}

diesel::table! {
    api_exports (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        request -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        text -> Nullable<Text>,
        safe -> Nullable<Bool>,
    }
}

diesel::table! {
    api_key_parameters (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        default -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    ar_internal_metadata (key) {
        key -> Varchar,
        value -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    category_values (id) {
        id -> Int4,
        options_category_id -> Nullable<Int4>,
        value_ru -> Nullable<Varchar>,
        value_en -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_accesses (id) {
        id -> Int8,
        finish_date -> Nullable<Date>,
        user_id -> Nullable<Int8>,
        allowed_by_id -> Nullable<Int8>,
        for_type -> Nullable<Varchar>,
        for_id -> Nullable<Int8>,
        state -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_api_logs (id) {
        id -> Int8,
        virtual_machine_id -> Nullable<Int8>,
        item_id -> Nullable<Int8>,
        log -> Nullable<Text>,
        action -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_conditions (id) {
        id -> Int8,
        from_type -> Nullable<Varchar>,
        from_id -> Nullable<Int8>,
        to_type -> Nullable<Varchar>,
        to_id -> Nullable<Int8>,
        from_multiplicity -> Nullable<Varchar>,
        to_multiplicity -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_item_links (id) {
        id -> Int8,
        from_id -> Nullable<Int8>,
        to_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_items (id) {
        id -> Int8,
        template_id -> Nullable<Int8>,
        item_id -> Nullable<Int8>,
        holder_type -> Nullable<Varchar>,
        holder_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_requests (id) {
        id -> Int8,
        comment -> Nullable<Text>,
        admin_comment -> Nullable<Text>,
        finish_date -> Nullable<Date>,
        created_by_id -> Nullable<Int8>,
        access_id -> Nullable<Int8>,
        for_type -> Nullable<Varchar>,
        for_id -> Nullable<Int8>,
        status -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_resource_items (id) {
        id -> Int8,
        resource_id -> Nullable<Int8>,
        item_id -> Nullable<Int8>,
        value -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_resource_kinds (id) {
        id -> Int8,
        name_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
        description_en -> Nullable<Text>,
        description_ru -> Nullable<Text>,
        help_en -> Nullable<Varchar>,
        help_ru -> Nullable<Varchar>,
        measurement_ru -> Nullable<Varchar>,
        measurement_en -> Nullable<Varchar>,
        identity -> Nullable<Varchar>,
        content_type -> Nullable<Int4>,
        template_kind_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_resources (id) {
        id -> Int8,
        resource_kind_id -> Nullable<Int8>,
        template_id -> Nullable<Int8>,
        value -> Nullable<Varchar>,
        editable -> Nullable<Bool>,
        min -> Nullable<Numeric>,
        max -> Nullable<Numeric>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_template_kinds (id) {
        id -> Int8,
        name_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
        description_ru -> Nullable<Text>,
        description_en -> Nullable<Text>,
        cloud_class -> Nullable<Varchar>,
        parent_id -> Nullable<Int4>,
        lft -> Int4,
        rgt -> Int4,
        depth -> Int4,
        children_count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_templates (id) {
        id -> Int8,
        template_kind_id -> Nullable<Int8>,
        name_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
        description_ru -> Nullable<Text>,
        description_en -> Nullable<Text>,
        identity -> Nullable<Int4>,
        new_requests -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cloud_computing_virtual_machines (id) {
        id -> Int8,
        identity -> Nullable<Int4>,
        item_id -> Nullable<Int8>,
        inner_address -> Nullable<Varchar>,
        internet_address -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        lcm_state -> Nullable<Varchar>,
        last_info -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments_comments (id) {
        id -> Int4,
        text -> Nullable<Text>,
        attachable_id -> Int4,
        attachable_type -> Varchar,
        user_id -> Int4,
        context_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments_context_groups (id) {
        id -> Int4,
        context_id -> Int4,
        group_id -> Int4,
        type_ab -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments_contexts (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    comments_file_attachments (id) {
        id -> Int4,
        file -> Nullable<Varchar>,
        description -> Nullable<Text>,
        attachable_id -> Int4,
        attachable_type -> Varchar,
        user_id -> Int4,
        context_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments_group_classes (id) {
        id -> Int4,
        class_name -> Nullable<Varchar>,
        obj_id -> Nullable<Int4>,
        group_id -> Nullable<Int4>,
        allow -> Bool,
        type_ab -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments_taggings (id) {
        id -> Int4,
        tag_id -> Nullable<Int4>,
        attachable_id -> Int4,
        attachable_type -> Varchar,
        user_id -> Nullable<Int4>,
        context_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments_tags (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_access_fields (id) {
        id -> Int4,
        access_id -> Nullable<Int4>,
        quota -> Nullable<Int4>,
        used -> Nullable<Int4>,
        quota_kind_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_accesses (id) {
        id -> Int4,
        project_id -> Int4,
        cluster_id -> Int4,
        state -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        project_group_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_bot_links (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        token -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    core_cities (id) {
        id -> Int4,
        country_id -> Nullable<Int4>,
        title_ru -> Nullable<Varchar>,
        title_en -> Nullable<Varchar>,
        checked -> Nullable<Bool>,
    }
}

diesel::table! {
    core_cluster_logs (id) {
        id -> Int4,
        cluster_id -> Int4,
        message -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        project_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_cluster_quotas (id) {
        id -> Int4,
        cluster_id -> Int4,
        value -> Nullable<Int4>,
        quota_kind_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_clusters (id) {
        id -> Int4,
        name_ru -> Varchar,
        host -> Varchar,
        description -> Nullable<Text>,
        public_key -> Nullable<Text>,
        private_key -> Nullable<Text>,
        admin_login -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        available_for_work -> Nullable<Bool>,
        name_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_countries (id) {
        id -> Int4,
        title_ru -> Nullable<Varchar>,
        title_en -> Nullable<Varchar>,
        checked -> Nullable<Bool>,
    }
}

diesel::table! {
    core_credentials (id) {
        id -> Int4,
        user_id -> Int4,
        state -> Nullable<Varchar>,
        name -> Varchar,
        public_key -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    core_critical_technologies (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_critical_technologies_per_projects (id) {
        id -> Int4,
        critical_technology_id -> Nullable<Int4>,
        project_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_department_mergers (id) {
        id -> Int4,
        source_department_id -> Nullable<Int4>,
        to_organization_id -> Nullable<Int4>,
        to_department_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    core_direction_of_sciences (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_direction_of_sciences_per_projects (id) {
        id -> Int4,
        direction_of_science_id -> Nullable<Int4>,
        project_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_employment_position_fields (id) {
        id -> Int4,
        employment_position_name_id -> Nullable<Int4>,
        name_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    core_employment_position_names (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        autocomplete -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_employment_positions (id) {
        id -> Int4,
        employment_id -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        value -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        employment_position_name_id -> Nullable<Int4>,
        field_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_employments (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        organization_id -> Nullable<Int4>,
        primary -> Nullable<Bool>,
        state -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        organization_department_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_group_of_research_areas (id) {
        id -> Int8,
        name_en -> Nullable<Varchar>,
        name_ru -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    core_members (id) {
        id -> Int4,
        user_id -> Int4,
        project_id -> Int4,
        owner -> Nullable<Bool>,
        login -> Nullable<Varchar>,
        project_access_state -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        organization_id -> Nullable<Int4>,
        organization_department_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_notice_show_options (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        notice_id -> Nullable<Int8>,
        hidden -> Bool,
        resolved -> Bool,
        answer -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_notices (id) {
        id -> Int4,
        sourceable_type -> Nullable<Varchar>,
        sourceable_id -> Nullable<Int4>,
        linkable_type -> Nullable<Varchar>,
        linkable_id -> Nullable<Int4>,
        message -> Nullable<Text>,
        count -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        category -> Nullable<Int4>,
        kind -> Nullable<Varchar>,
        show_from -> Nullable<Timestamp>,
        show_till -> Nullable<Timestamp>,
        active -> Nullable<Int4>,
    }
}

diesel::table! {
    core_organization_departments (id) {
        id -> Int4,
        organization_id -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        checked -> Nullable<Bool>,
    }
}

diesel::table! {
    core_organization_kinds (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        departments_required -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_organizations (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        abbreviation -> Nullable<Varchar>,
        kind_id -> Nullable<Int4>,
        country_id -> Nullable<Int4>,
        city_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        checked -> Nullable<Bool>,
    }
}

diesel::table! {
    core_partitions (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        cluster_id -> Nullable<Int4>,
        resources -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_project_cards (id) {
        id -> Int4,
        project_id -> Nullable<Int4>,
        name -> Nullable<Text>,
        en_name -> Nullable<Text>,
        driver -> Nullable<Text>,
        en_driver -> Nullable<Text>,
        strategy -> Nullable<Text>,
        en_strategy -> Nullable<Text>,
        objective -> Nullable<Text>,
        en_objective -> Nullable<Text>,
        impact -> Nullable<Text>,
        en_impact -> Nullable<Text>,
        usage -> Nullable<Text>,
        en_usage -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    core_project_invitations (id) {
        id -> Int4,
        project_id -> Int4,
        user_fio -> Varchar,
        user_email -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        language -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_project_kinds (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_projects (id) {
        id -> Int4,
        title -> Varchar,
        state -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        organization_id -> Nullable<Int4>,
        organization_department_id -> Nullable<Int4>,
        kind_id -> Nullable<Int4>,
        first_activation_at -> Nullable<Timestamp>,
        finished_at -> Nullable<Timestamp>,
        estimated_finish_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    core_quota_kinds (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        measurement_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
        measurement_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    core_request_fields (id) {
        id -> Int4,
        request_id -> Int4,
        value -> Nullable<Int4>,
        quota_kind_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_requests (id) {
        id -> Int4,
        project_id -> Int4,
        cluster_id -> Int4,
        state -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        cpu_hours -> Nullable<Int4>,
        gpu_hours -> Nullable<Int4>,
        hdd_size -> Nullable<Int4>,
        group_name -> Nullable<Varchar>,
        creator_id -> Nullable<Int4>,
        comment -> Nullable<Text>,
        reason -> Nullable<Text>,
        changed_by_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_research_areas (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        old_group -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name_en -> Nullable<Varchar>,
        group_id -> Nullable<Int8>,
    }
}

diesel::table! {
    core_research_areas_per_projects (id) {
        id -> Int4,
        research_area_id -> Nullable<Int4>,
        project_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_sureties (id) {
        id -> Int4,
        project_id -> Nullable<Int4>,
        state -> Nullable<Varchar>,
        comment -> Nullable<Varchar>,
        boss_full_name -> Nullable<Varchar>,
        boss_position -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        document -> Nullable<Varchar>,
        author_id -> Nullable<Int4>,
        reason -> Nullable<Text>,
        changed_by_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_surety_members (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        surety_id -> Nullable<Int4>,
        organization_id -> Nullable<Int4>,
        organization_department_id -> Nullable<Int4>,
    }
}

diesel::table! {
    core_surety_scans (id) {
        id -> Int4,
        surety_id -> Nullable<Int4>,
        image -> Nullable<Varchar>,
    }
}

diesel::table! {
    delayed_jobs (id) {
        id -> Int4,
        priority -> Int4,
        attempts -> Int4,
        handler -> Text,
        last_error -> Nullable<Text>,
        run_at -> Nullable<Timestamp>,
        locked_at -> Nullable<Timestamp>,
        failed_at -> Nullable<Timestamp>,
        locked_by -> Nullable<Varchar>,
        queue -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    face_menu_item_prefs (id) {
        id -> Int8,
        position -> Nullable<Int4>,
        menu -> Nullable<Varchar>,
        key -> Nullable<Varchar>,
        user_id -> Nullable<Int8>,
        admin -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    face_users_menus (id) {
        id -> Int8,
        menu -> Nullable<Varchar>,
        user_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        weight -> Nullable<Int4>,
        system -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    hardware_items (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
        description_ru -> Nullable<Text>,
        description_en -> Nullable<Text>,
        lock_version -> Nullable<Int4>,
        kind_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    hardware_items_states (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
        state_id -> Nullable<Int4>,
        reason_en -> Nullable<Text>,
        reason_ru -> Nullable<Text>,
        description_en -> Nullable<Text>,
        description_ru -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    hardware_kinds (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
        description_ru -> Nullable<Text>,
        description_en -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    hardware_states (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
        description_ru -> Nullable<Text>,
        description_en -> Nullable<Text>,
        lock_version -> Nullable<Int4>,
        kind_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    hardware_states_links (id) {
        id -> Int4,
        from_id -> Nullable<Int4>,
        to_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    jobstat_data_types (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    jobstat_digest_float_data (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        job_id -> Nullable<Int8>,
        value -> Nullable<Float8>,
        time -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    jobstat_digest_string_data (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        job_id -> Nullable<Int8>,
        value -> Nullable<Varchar>,
        time -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    jobstat_float_data (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        job_id -> Nullable<Int8>,
        value -> Nullable<Float8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    jobstat_job_mail_filters (id) {
        id -> Int4,
        condition -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    jobstat_jobs (id) {
        id -> Int4,
        cluster -> Nullable<Varchar>,
        drms_job_id -> Nullable<Int8>,
        drms_task_id -> Nullable<Int8>,
        login -> Nullable<Varchar>,
        partition -> Nullable<Varchar>,
        submit_time -> Nullable<Timestamp>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        timelimit -> Nullable<Int8>,
        command -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        num_cores -> Nullable<Int8>,
        num_nodes -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        nodelist -> Nullable<Text>,
        initiator_id -> Nullable<Int4>,
    }
}

diesel::table! {
    jobstat_string_data (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        job_id -> Nullable<Int8>,
        value -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    options (id) {
        id -> Int4,
        owner_id -> Nullable<Int4>,
        owner_type -> Nullable<Varchar>,
        name_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
        value_ru -> Nullable<Text>,
        value_en -> Nullable<Text>,
        category_value_id -> Nullable<Int4>,
        options_category_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        admin -> Nullable<Bool>,
    }
}

diesel::table! {
    options_categories (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pack_accesses (id) {
        id -> Int4,
        version_id -> Nullable<Int4>,
        who_id -> Nullable<Int4>,
        who_type -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        created_by_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        end_lic -> Nullable<Date>,
        new_end_lic -> Nullable<Date>,
        allowed_by_id -> Nullable<Int4>,
        lock_version -> Int4,
        new_end_lic_forever -> Nullable<Bool>,
        to_type -> Nullable<Varchar>,
        to_id -> Nullable<Int8>,
    }
}

diesel::table! {
    pack_category_values (id) {
        id -> Int4,
        options_category_id -> Nullable<Int4>,
        value_ru -> Nullable<Varchar>,
        value_en -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pack_clustervers (id) {
        id -> Int4,
        core_cluster_id -> Nullable<Int4>,
        version_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        active -> Nullable<Bool>,
        path -> Nullable<Varchar>,
    }
}

diesel::table! {
    pack_options_categories (id) {
        id -> Int4,
        category_ru -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        category_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    pack_packages (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        description_ru -> Nullable<Text>,
        deleted -> Bool,
        description_en -> Nullable<Text>,
        name_en -> Nullable<Varchar>,
        accesses_to_package -> Nullable<Bool>,
    }
}

diesel::table! {
    pack_version_options (id) {
        id -> Int4,
        version_id -> Nullable<Int4>,
        name_ru -> Nullable<Varchar>,
        value_ru -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name_en -> Nullable<Varchar>,
        value_en -> Nullable<Varchar>,
        category_value_id -> Nullable<Int4>,
        options_category_id -> Nullable<Int4>,
    }
}

diesel::table! {
    pack_versions (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        description_ru -> Nullable<Text>,
        package_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        cost -> Nullable<Int4>,
        end_lic -> Nullable<Date>,
        state -> Nullable<Varchar>,
        lock_col -> Int4,
        deleted -> Bool,
        service -> Bool,
        delete_on_expire -> Bool,
        ticket_id -> Nullable<Int4>,
        description_en -> Nullable<Text>,
        name_en -> Nullable<Varchar>,
        ticket_created -> Nullable<Bool>,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int4,
        action -> Nullable<Varchar>,
        subject_class -> Nullable<Varchar>,
        group_id -> Nullable<Int4>,
        available -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        subject_id -> Nullable<Int4>,
    }
}

diesel::table! {
    profiles (id) {
        id -> Int4,
        user_id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        middle_name -> Nullable<Varchar>,
        about -> Nullable<Text>,
        receive_info_mails -> Nullable<Bool>,
        receive_special_mails -> Nullable<Bool>,
    }
}

diesel::table! {
    schema_migrations (version) {
        version -> Varchar,
    }
}

diesel::table! {
    sessions_managers (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        session_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sessions_projects_in_sessions (id) {
        id -> Int4,
        session_id -> Nullable<Int4>,
        project_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sessions_report_materials (id) {
        id -> Int4,
        materials -> Nullable<Varchar>,
        materials_content_type -> Nullable<Varchar>,
        materials_file_size -> Nullable<Int4>,
        report_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sessions_report_replies (id) {
        id -> Int4,
        report_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        message -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sessions_report_submit_denial_reasons (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    sessions_reports (id) {
        id -> Int4,
        session_id -> Nullable<Int4>,
        project_id -> Nullable<Int4>,
        author_id -> Nullable<Int4>,
        expert_id -> Nullable<Int4>,
        state -> Nullable<Varchar>,
        materials -> Nullable<Varchar>,
        materials_file_name -> Nullable<Varchar>,
        materials_content_type -> Nullable<Varchar>,
        materials_file_size -> Nullable<Int4>,
        materials_updated_at -> Nullable<Timestamp>,
        illustration_points -> Nullable<Int4>,
        summary_points -> Nullable<Int4>,
        statement_points -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        submit_denial_reason_id -> Nullable<Int4>,
        submit_denial_description -> Nullable<Text>,
    }
}

diesel::table! {
    sessions_sessions (id) {
        id -> Int4,
        state -> Nullable<Varchar>,
        description_ru -> Nullable<Text>,
        motivation_ru -> Nullable<Text>,
        started_at -> Nullable<Timestamp>,
        ended_at -> Nullable<Timestamp>,
        receiving_to -> Nullable<Timestamp>,
        description_en -> Nullable<Text>,
        motivation_en -> Nullable<Text>,
    }
}

diesel::table! {
    sessions_stats (id) {
        id -> Int4,
        session_id -> Nullable<Int4>,
        survey_field_id -> Nullable<Int4>,
        group_by -> Nullable<Varchar>,
        weight -> Nullable<Int4>,
        organization_id -> Nullable<Int4>,
        cache -> Nullable<Text>,
    }
}

diesel::table! {
    sessions_survey_fields (id) {
        id -> Int4,
        survey_id -> Nullable<Int4>,
        kind -> Nullable<Varchar>,
        collection -> Nullable<Text>,
        max_values -> Nullable<Int4>,
        weight -> Nullable<Int4>,
        name_ru -> Nullable<Text>,
        required -> Nullable<Bool>,
        entity -> Nullable<Varchar>,
        strict_collection -> Nullable<Bool>,
        hint_ru -> Nullable<Varchar>,
        reference_type -> Nullable<Varchar>,
        regexp -> Nullable<Varchar>,
        hint_en -> Nullable<Varchar>,
        name_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    sessions_survey_kinds (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    sessions_survey_values (id) {
        id -> Int4,
        value -> Nullable<Text>,
        survey_field_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        user_survey_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sessions_surveys (id) {
        id -> Int4,
        session_id -> Nullable<Int4>,
        kind_id -> Nullable<Int4>,
        name_ru -> Nullable<Varchar>,
        only_for_project_owners -> Nullable<Bool>,
        name_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    sessions_user_surveys (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        session_id -> Nullable<Int4>,
        survey_id -> Nullable<Int4>,
        project_id -> Nullable<Int4>,
        state -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    statistics_organization_stats (id) {
        id -> Int4,
        kind -> Nullable<Varchar>,
        data -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    statistics_project_stats (id) {
        id -> Int4,
        kind -> Nullable<Varchar>,
        data -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    statistics_session_stats (id) {
        id -> Int4,
        kind -> Nullable<Varchar>,
        data -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    statistics_user_stats (id) {
        id -> Int4,
        kind -> Nullable<Varchar>,
        data -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    support_field_options (id) {
        id -> Int8,
        field_id -> Nullable<Int8>,
        name_ru -> Nullable<Text>,
        name_en -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    support_field_values (id) {
        id -> Int4,
        field_id -> Nullable<Int4>,
        ticket_id -> Nullable<Int4>,
        value -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        topics_field_id -> Nullable<Int8>,
    }
}

diesel::table! {
    support_fields (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        hint_ru -> Nullable<Varchar>,
        required -> Nullable<Bool>,
        contains_source_code -> Nullable<Bool>,
        url -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name_en -> Nullable<Varchar>,
        hint_en -> Nullable<Varchar>,
        model_collection -> Nullable<Varchar>,
        kind -> Nullable<Int4>,
        search -> Nullable<Bool>,
    }
}

diesel::table! {
    support_replies (id) {
        id -> Int4,
        author_id -> Nullable<Int4>,
        ticket_id -> Nullable<Int4>,
        message -> Nullable<Text>,
        attachment -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        attachment_file_name -> Nullable<Varchar>,
        attachment_content_type -> Nullable<Varchar>,
        attachment_file_size -> Nullable<Int4>,
        attachment_updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    support_reply_templates (id) {
        id -> Int4,
        subject_ru -> Nullable<Varchar>,
        message_ru -> Nullable<Text>,
        subject_en -> Nullable<Varchar>,
        message_en -> Nullable<Text>,
    }
}

diesel::table! {
    support_tags (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name_en -> Nullable<Varchar>,
    }
}

diesel::table! {
    support_tickets (id) {
        id -> Int4,
        topic_id -> Nullable<Int4>,
        project_id -> Nullable<Int4>,
        cluster_id -> Nullable<Int4>,
        surety_id -> Nullable<Int4>,
        reporter_id -> Nullable<Int4>,
        subject -> Nullable<Varchar>,
        message -> Nullable<Text>,
        state -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        attachment -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        responsible_id -> Nullable<Int4>,
        attachment_file_name -> Nullable<Varchar>,
        attachment_content_type -> Nullable<Varchar>,
        attachment_file_size -> Nullable<Int4>,
        attachment_updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    support_tickets_subscribers (id) {
        id -> Int4,
        ticket_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    support_tickets_tags (id) {
        id -> Int4,
        ticket_id -> Nullable<Int4>,
        tag_id -> Nullable<Int4>,
    }
}

diesel::table! {
    support_topics (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        parent_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name_en -> Nullable<Varchar>,
        template_en -> Nullable<Text>,
        template_ru -> Nullable<Text>,
        visible_on_create -> Nullable<Bool>,
    }
}

diesel::table! {
    support_topics_fields (id) {
        id -> Int4,
        topic_id -> Nullable<Int4>,
        field_id -> Nullable<Int4>,
        required -> Nullable<Bool>,
    }
}

diesel::table! {
    support_topics_tags (id) {
        id -> Int4,
        topic_id -> Nullable<Int4>,
        tag_id -> Nullable<Int4>,
    }
}

diesel::table! {
    support_user_topics (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        topic_id -> Nullable<Int4>,
        required -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_groups (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        group_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        crypted_password -> Nullable<Varchar>,
        salt -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        activation_state -> Nullable<Varchar>,
        activation_token -> Nullable<Varchar>,
        activation_token_expires_at -> Nullable<Timestamp>,
        remember_me_token -> Nullable<Varchar>,
        remember_me_token_expires_at -> Nullable<Timestamp>,
        reset_password_token -> Nullable<Varchar>,
        reset_password_token_expires_at -> Nullable<Timestamp>,
        reset_password_email_sent_at -> Nullable<Timestamp>,
        access_state -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        last_login_at -> Nullable<Timestamp>,
        last_logout_at -> Nullable<Timestamp>,
        last_activity_at -> Nullable<Timestamp>,
        last_login_from_ip_address -> Nullable<Varchar>,
        language -> Nullable<Varchar>,
    }
}

diesel::table! {
    versions (id) {
        id -> Int4,
        item_type -> Varchar,
        item_id -> Int8,
        event -> Varchar,
        whodunnit -> Nullable<Int4>,
        object -> Nullable<Text>,
        session_id -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        object_changes -> Nullable<Text>,
    }
}

diesel::table! {
    wiki_pages (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        content_ru -> Nullable<Text>,
        url -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name_en -> Nullable<Varchar>,
        content_en -> Nullable<Text>,
    }
}

diesel::table! {
    wikiplus_images (id) {
        id -> Int4,
        image -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    wikiplus_pages (id) {
        id -> Int4,
        name_ru -> Nullable<Varchar>,
        content_ru -> Nullable<Text>,
        url -> Nullable<Varchar>,
        show_all -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name_en -> Nullable<Varchar>,
        content_en -> Nullable<Text>,
        sortid -> Nullable<Numeric>,
        mainpage_id -> Nullable<Int4>,
        image -> Nullable<Varchar>,
    }
}


diesel::allow_tables_to_appear_in_same_query!(
    active_storage_attachments,
    active_storage_blobs,
    announcement_recipients,
    announcements,
    api_access_keys,
    api_access_keys_exports,
    api_exports,
    api_key_parameters,
    ar_internal_metadata,
    category_values,
    cloud_computing_accesses,
    cloud_computing_api_logs,
    cloud_computing_conditions,
    cloud_computing_item_links,
    cloud_computing_items,
    cloud_computing_requests,
    cloud_computing_resource_items,
    cloud_computing_resource_kinds,
    cloud_computing_resources,
    cloud_computing_template_kinds,
    cloud_computing_templates,
    cloud_computing_virtual_machines,
    comments_comments,
    comments_context_groups,
    comments_contexts,
    comments_file_attachments,
    comments_group_classes,
    comments_taggings,
    comments_tags,
    core_access_fields,
    core_accesses,
    core_bot_links,
    core_cities,
    core_cluster_logs,
    core_cluster_quotas,
    core_clusters,
    core_countries,
    core_credentials,
    core_critical_technologies,
    core_critical_technologies_per_projects,
    core_department_mergers,
    core_direction_of_sciences,
    core_direction_of_sciences_per_projects,
    core_employment_position_fields,
    core_employment_position_names,
    core_employment_positions,
    core_employments,
    core_group_of_research_areas,
    core_members,
    core_notice_show_options,
    core_notices,
    core_organization_departments,
    core_organization_kinds,
    core_organizations,
    core_partitions,
    core_project_cards,
    core_project_invitations,
    core_project_kinds,
    core_projects,
    core_quota_kinds,
    core_request_fields,
    core_requests,
    core_research_areas,
    core_research_areas_per_projects,
    core_sureties,
    core_surety_members,
    core_surety_scans,
    delayed_jobs,
    face_menu_item_prefs,
    face_users_menus,
    groups,
    hardware_items,
    hardware_items_states,
    hardware_kinds,
    hardware_states,
    hardware_states_links,
    jobstat_data_types,
    jobstat_digest_float_data,
    jobstat_digest_string_data,
    jobstat_float_data,
    jobstat_job_mail_filters,
    jobstat_jobs,
    jobstat_string_data,
    options,
    options_categories,
    pack_accesses,
    pack_category_values,
    pack_clustervers,
    pack_options_categories,
    pack_packages,
    pack_version_options,
    pack_versions,
    permissions,
    profiles,
    schema_migrations,
    sessions_managers,
    sessions_projects_in_sessions,
    sessions_report_materials,
    sessions_report_replies,
    sessions_report_submit_denial_reasons,
    sessions_reports,
    sessions_sessions,
    sessions_stats,
    sessions_survey_fields,
    sessions_survey_kinds,
    sessions_survey_values,
    sessions_surveys,
    sessions_user_surveys,
    statistics_organization_stats,
    statistics_project_stats,
    statistics_session_stats,
    statistics_user_stats,
    support_field_options,
    support_field_values,
    support_fields,
    support_replies,
    support_reply_templates,
    support_tags,
    support_tickets,
    support_tickets_subscribers,
    support_tickets_tags,
    support_topics,
    support_topics_fields,
    support_topics_tags,
    support_user_topics,
    user_groups,
    users,
    versions,
    wiki_pages,
    wikiplus_images,
    wikiplus_pages,
);
