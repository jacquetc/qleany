// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#include "database/database_context.h"
#include "QtSql/qsqlerror.h"
#include <QSqlDatabase>
#include <QSqlQuery>
#include <QString>
#include <QTemporaryFile>

using namespace DatabaseTest::Persistence::Database;

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

DatabaseTest::Result<void> DatabaseContext::init()
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
    QString connectionName = "Thread_%1"_L1.arg(QString::number(uintptr_t(QThread::currentThreadId())));
    if (!QSqlDatabase::contains(connectionName))
    {
        QSqlDatabase database = QSqlDatabase::addDatabase("QSQLITE"_L1, connectionName);
        database.setDatabaseName(m_databaseName);
        if (!database.open())
        {
            QSqlDatabase::removeDatabase(connectionName);
            qDebug() << Q_FUNC_INFO << "sql_error" << database.lastError().text();
        }

        // List of PRAGMA statements to execute for the new connection
        QStringList pragmas = {
            QStringLiteral("PRAGMA case_sensitive_like=true"), QStringLiteral("PRAGMA journal_mode=MEMORY"),
            QStringLiteral("PRAGMA temp_store=MEMORY"),        QStringLiteral("PRAGMA locking_mode=NORMAL"),
            QStringLiteral("PRAGMA synchronous=OFF"),          QStringLiteral("PRAGMA recursive_triggers=ON"),
            QStringLiteral("PRAGMA foreign_keys=ON")};

        QSqlQuery pragmaQuery(database);
        for (const QString &pragma : pragmas)
        {
            if (!pragmaQuery.exec(pragma))
            {
                qDebug() << Q_FUNC_INFO << "pragma_error" << pragma << pragmaQuery.lastError().text();
                // Decide on error handling: continue, abort, or some other strategy
            }
        }
    }

    // qDebug() << QSqlDatabase::connectionNames();

    return QSqlDatabase::database(connectionName);
}

//-------------------------------------------------

DatabaseTest::Result<QString> DatabaseContext::createEmptyDatabase()
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
        qDebug() << "database name" << m_databaseName;

        QSqlDatabase sqlDb = getConnection();

        // start a transaction
        sqlDb.transaction();

        // execute each table creation as a single query within the transaction
        QSqlQuery query(sqlDb);

        // entity tables
        QList<QString> entityTableSqls = m_creationSqlHash.values("entity_table"_L1);

        for (const QString &string : entityTableSqls)
        {
            if (!query.prepare(string))
            {
                return Result<QString>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error",
                                                   query.lastError().text().toLatin1().constData(),
                                                   string.toLatin1().constData()));
            }
            if (!query.exec())
            {
                return Result<QString>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error",
                                                   query.lastError().text().toLatin1().constData(),
                                                   string.toLatin1().constData()));
            }
        }

        // junction tables
        QList<QString> junctionTableSqls = m_creationSqlHash.values("junction_table"_L1);

        for (const QString &string : junctionTableSqls)
        {
            if (!query.prepare(string))
            {
                return Result<QString>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error",
                                                   query.lastError().text().toLatin1().constData(),
                                                   string.toLatin1().constData()));
            }
            if (!query.exec())
            {
                return Result<QString>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error",
                                                   query.lastError().text().toLatin1().constData(),
                                                   string.toLatin1().constData()));
            }
        }

        // database optimization options
        QStringList optimization;
        optimization << QStringLiteral("PRAGMA case_sensitive_like=true")
                     << QStringLiteral("PRAGMA journal_mode=MEMORY") << QStringLiteral("PRAGMA temp_store=MEMORY")
                     << QStringLiteral("PRAGMA locking_mode=NORMAL") << QStringLiteral("PRAGMA synchronous = OFF")
                     << QStringLiteral("PRAGMA recursive_triggers = ON") << QStringLiteral("PRAGMA foreign_keys = ON");

        // execute each optimization option as a single query within the transaction

        for (const QString &string : std::as_const(optimization))
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

void DatabaseContext::appendCreationSql(const char *type, const QString &sql)
{
    m_creationSqlHash.insert(QString::fromLatin1(type), sql);
}