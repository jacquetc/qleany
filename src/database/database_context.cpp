#include "qleany/database/database_context.h"
#include "QtSql/qsqlerror.h"
#include <QSqlDatabase>
#include <QSqlQuery>
#include <QString>
#include <QTemporaryFile>

using namespace Qleany::Database;

DatabaseContext::DatabaseContext()
{
}

DatabaseContext::~DatabaseContext()
{

    // remove m_databaseName file
    QFile::remove(m_databaseName);

    // remove all connection
    QStringList connectionNames = QSqlDatabase::connectionNames();
    for (const QString &connectionName : connectionNames)
    {
        QSqlDatabase::removeDatabase(connectionName);
    }
}

//-------------------------------------------------

Qleany::Result<void> DatabaseContext::init()
{

    Result<QString> databaseNameResult = createEmptyDatabase();

    if (databaseNameResult.isError())
    {
        return Result<void>(databaseNameResult.error());
    }
    return Result<void>();
}

//-------------------------------------------------

QSqlDatabase DatabaseContext::getConnection()
{
    QMutexLocker locker(&mutex);
    QString connectionName = QString("Thread_%1").arg(uintptr_t(QThread::currentThreadId()));
    if (!QSqlDatabase::contains(connectionName))
    {
        QSqlDatabase database = QSqlDatabase::addDatabase("QSQLITE", connectionName);
        database.setDatabaseName(m_databaseName);
        if (!database.open())
        {
            QSqlDatabase::removeDatabase(connectionName);
            qDebug() << Q_FUNC_INFO << "sql_error" << database.lastError().text();
        }
    }
    // qDebug() << QSqlDatabase::connectionNames();

    return QSqlDatabase::database(connectionName);
}

//-------------------------------------------------

Qleany::Result<QString> DatabaseContext::createEmptyDatabase()
{
    QString databaseName;

    // create a temporary file to copy the database to
    QTemporaryFile tempFile;
    tempFile.open();
    tempFile.setAutoRemove(false);
    QString tempFileName = tempFile.fileName();

    {
        m_databaseName = tempFileName;
        databaseName = m_databaseName;
        // qDebug() << m_databaseName;

        QSqlDatabase sqlDb = getConnection();

        // start a transaction
        sqlDb.transaction();

        // execute each table creation as a single query within the transaction
        QSqlQuery query(sqlDb);

        // entity tables
        QList<QString> entityTableSqls = m_creationSqlHash.values("entity_table");

        for (const QString &string : entityTableSqls)
        {
            if (!query.prepare(string))
            {
                return Result<QString>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), string));
            }
            if (!query.exec())
            {
                return Result<QString>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), string));
            }
        }

        // junction tables
        QList<QString> junctionTableSqls = m_creationSqlHash.values("junction_table");

        for (const QString &string : junctionTableSqls)
        {
            if (!query.prepare(string))
            {
                return Result<QString>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), string));
            }
            if (!query.exec())
            {
                return Result<QString>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), string));
            }
        }

        // database optimization options
        QStringList optimization;
        optimization << QStringLiteral("PRAGMA case_sensitive_like=true")
                     << QStringLiteral("PRAGMA journal_mode=MEMORY") << QStringLiteral("PRAGMA temp_store=MEMORY")
                     << QStringLiteral("PRAGMA locking_mode=NORMAL") << QStringLiteral("PRAGMA synchronous = OFF")
                     << QStringLiteral("PRAGMA recursive_triggers = ON") << QStringLiteral("PRAGMA foreign_keys = ON");

        // execute each optimization option as a single query within the transaction

        for (const QString &string : qAsConst(optimization))
        {
            query.prepare(string);
            query.exec();
        }

        sqlDb.commit();
    }

    // return the name of the copied database file
    return Result<QString>(databaseName);
}

//---------------------------------------------------------

void Qleany::Database::DatabaseContext::appendCreationSql(const QString &type, const QString &sql)
{
    m_creationSqlHash.insert(type, sql);
}
