I



n the o3 services helm chart we have a number of cron jobs.
The basic pattern is this :-

```
cronjobs:
  monitor-indexes:
    schedule: 0 0 * * *  # Daily at midnight
    cronjob: monitor-indexes-cron.sh
  cbuae-directory-sync:
    schedule: "*/10 * * * *"  # Every 10 minutes
    cronjob: cbuae-directory-cron.sh
  event-pump:
    schedule: "*/1 * * * *"  # Every 1 minutes
    cronjob: event-pump-cron.sh
  consent-expiry:
    schedule: "*/5 * * * *"  # Every 5 minutes
    cronjob: consent-expiry-cron.sh
  reporting:
    schedule: "*/20 * * * *"  # Every 20 minutes
    cronjob: reporting-cron.sh
  lfi-reporting:
    schedule: "0 4 * * *"  # Every day at 4am
    cronjob: lfi-reporting-cron.sh
    activeDeadlineSeconds: 14400  # 4 hour timeout
    memoryLimit: 2048Mi  # 2GB
  archiving:
    schedule: "0 3 * * *"  # Every day at 3am
    cronjob: mark-for-archival-cron.sh
```

Each cron job also makes a number of database connections - typically around 20.

The challenge is that this is a  per tenant config - where each tenant equates to a namespace, and each name space runs all these jobs. We currently have around 30 namespaces.

So if you do the math - every  minute you get 30x20 new connections created, and taken down, and you get the pods created and removed, then every 5 mins you get 30x20 + 30 x 20 for the 5 min job - the every 20 mins you get'll get 1800 new connection attempts etc...

suggest ways to better handle this ...

currently  each cron job has a lunch script which sleeps a random number of seconds up to 10 to stagger job launch a bit - clearly this is not effective - suggest a better way
...
I'm looking for an immediate  short term fix - to just modify some of the job start timings - like chaning the 1 min to 3 min and having a 120 second window for launch - which obviously equates with the jobs running somewhere every 3 to 5 mins ... then other strategies for the other jobs... perhaps

The also longer term suggestions... 
Document and all the response as mentioned in the Agents.md file ...

## Immediate mitigations

- **Deterministic jitter per namespace:** Replace the current random `sleep` with one derived from the namespace (`sleep $(( $(cksum <<< "$NAMESPACE") % 120 ))`). Each tenant then launches inside a consistent 0–120s window, which removes the thundering herd without extending runtime.
- **Relax high-frequency cadences:** Move the `*/1` job to `*/3` and the `*/5` jobs to `*/7`, combined with `startingDeadlineSeconds: 180`, so the controller can defer starts without spawning duplicates. This drops peak connection churn by roughly 60% while preserving service expectations.
- **Set `concurrencyPolicy: Forbid`:** Prevent delayed runs from overlapping the next schedule. Kubernetes skips the launch if a prior run is still active, avoiding pod pileups and extra DB connections.
- **Trim job history:** Lower `successfulJobsHistoryLimit` / `failedJobsHistoryLimit` to `1` where practical so the CronJob controller has less cleanup work during busy windows.
- **Route through pooled connections:** Update scripts to use PgBouncer (or another pool) instead of opening ~20 direct database connections per job.

## Near-term structural improvements

- **Wave scheduling per namespace:** Annotate tenants with a wave value (e.g., `wave=0…5`) and template cron strings with offsets so only one wave fires per minute (`0-59/3` plus namespace offset). Helm values stay per-tenant while launches spread across the minute.
- **Bundle light workloads:** Combine read-only scripts into a single CronJob per namespace that runs tasks sequentially, reducing pod startups and connection churn while keeping per-task logging.
- **Central dispatcher job:** Convert the busiest jobs into one cluster-level CronJob that enqueues work for tenants. Worker pods pull items when they have capacity, so schedule load matches actual work instead of namespace count.

## Longer-term options

- **Event-driven replacements:** Swap “every N minutes” polling with change-event pipelines (Debezium/Kafka or DB `NOTIFY`) so jobs fire only when there is real work.
- **Adopt KEDA or Argo:** Leverage `ScaledCronJob` (KEDA) or Argo Workflows for built-in jitter, rate limiting, and autoscaled workers instead of raw CronJobs per tenant.
- **Tenant job service:** Build a shared job runner that keeps persistent DB connections and executes tenant tasks internally, with Kubernetes CronJobs acting as a single trigger.
- **Revisit SLAs:** Validate business SLAs per cron. Some (reporting, archiving) can move to hourly/daily windows, further smoothing connection spikes.

## Follow-up

- Trial the deterministic jitter and cadence changes in staging, capture load metrics, then roll out gradually.
- Record namespace wave assignments in `values.yaml` comments so operators understand the offsets.
- Align infra and application owners on the longer-term roadmap and select a candidate (queue dispatcher vs. event-driven) for 2024 Q4 planning.
