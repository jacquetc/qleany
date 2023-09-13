#pragma once
#include "QtSql/qsqlerror.h"
#include "qleany/common/result.h"
#include "qleany/contracts/database/interface_database_context.h"
#include <QSqlDatabase>
#include <QSqlQuery>
#include <QString>
#include <QThreadPool>
#include <QUuid>

using namespace Qleany;

template <class T, class U> class DummyDatabaseContext : public Contracts::Database::InterfaceDatabaseContext
{
  public:
    DummyDatabaseContext();
    ~DummyDatabaseContext();
    Result<void> init() override;

  private:
    QString m_databaseName;
    QStringList m_entityClassNames;
    QMultiHash<QString, QString> m_creationSqlHash;

    // InterfaceDatabaseContext interface
  public:
    QSqlDatabase getConnection() override;

    // InterfaceDatabaseContext interface
  public:
    void appendCreationSql(const QString &type, const QString &sql) override;
};

template <class T, class U> void DummyDatabaseContext<T, U>::appendCreationSql(const QString &type, const QString &sql)
{
    m_creationSqlHash.insert(type, sql);
    qDebug() << sql;
}

template <class T, class U> DummyDatabaseContext<T, U>::DummyDatabaseContext()
{
    qRegisterMetaType<T>(T::staticMetaObject.className());

    qRegisterMetaType<U>(U::staticMetaObject.className());

    m_databaseName = ":memory:";
}

template <class T, class U> DummyDatabaseContext<T, U>::~DummyDatabaseContext()
{
}

template <class T, class U> Result<void> DummyDatabaseContext<T, U>::init()
{

    {
        QString databaseName = m_databaseName;
        // qDebug() << m_databaseName;

        QSqlDatabase sqlDb = getConnection();

        // start a transaction
        sqlDb.transaction();

        // execute each table creation as a single query within the transaction
        QSqlQuery query(sqlDb);

        // entity tables
        QStringList entityTableSqls = m_creationSqlHash.values("entity_table");
        entityTableSqls.removeDuplicates();

        for (const QString &string : entityTableSqls)
        {
            if (!query.prepare(string))
            {
                return Result<void>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), string));
            }
            if (!query.exec())
            {
                return Result<void>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), string));
            }
        }

        // junction tables
        QStringList junctionTableSqls = m_creationSqlHash.values("junction_table");
        junctionTableSqls.removeDuplicates();

        for (const QString &string : junctionTableSqls)
        {
            if (!query.prepare(string))
            {
                return Result<void>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), string));
            }
            if (!query.exec())
            {
                return Result<void>(QLN_ERROR_3(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), string));
            }
        }

        // database optimization options
        QStringList optimization;
        optimization << QStringLiteral("PRAGMA case_sensitive_like=true")
                     << QStringLiteral("PRAGMA journal_mode=MEMORY") << QStringLiteral("PRAGMA temp_store=MEMORY")
                     << QStringLiteral("PRAGMA locking_mode=NORMAL") << QStringLiteral("PRAGMA synchronous = OFF")
                     << QStringLiteral("PRAGMA recursive_triggers=true");

        // execute each optimization option as a single query within the transaction

        for (const QString &string : qAsConst(optimization))
        {
            query.prepare(string);
            query.exec();
        }

        sqlDb.commit();
    }

    return Result<void>();
}

template <class T, class U> QSqlDatabase DummyDatabaseContext<T, U>::getConnection()
{
    QString connectionName = QString("connectionName");
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
