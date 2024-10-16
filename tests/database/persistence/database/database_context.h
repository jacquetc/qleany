// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "result.h"
#include "database/interface_database_context.h"
#include "database_test_persistence_export.h"
#include <QHash>
#include <QMutexLocker>
#include <QReadWriteLock>
#include <QThreadPool>
#include <QUrl>

using namespace DatabaseTest::Contracts::Database;

namespace DatabaseTest::Persistence::Database
{

/**
 * @brief The DatabaseContext class represents the context for a internal database.
 */
class DATABASE_TEST_PERSISTENCE_EXPORT DatabaseContext : public InterfaceDatabaseContext
{
  public:
    /**
     * @brief Constructs a new DatabaseContext object.
     */
    explicit DatabaseContext();

    /**
     * @brief Destructor for the DatabaseContext object.
     */
    ~DatabaseContext();

    /**
     * @brief Initializes the internal database.
     * @return A Result object with a value of nullptr if successful, or an Error object if an error occurred.
     */
    Result<void> init() override;

    QSqlDatabase getConnection() override;

    void appendCreationSql(const char *type, const QString &sql) override;

  private:
    QMutex mutex;
    QUrl m_fileName;        /**< The file name of the internal database. */
    QString m_databaseName; /**< The name of the internal database. */

    QMultiHash<QString, QString> m_creationSqlHash;

    /**
     * @brief Loads the internal database from the given file name.
     * @param fileName The file name of the internal database to load.
     * @return A Result object with the name of the loaded database if successful, or an Error object if an error
     * occurred.
     */
    Result<QString> createEmptyDatabase();
    QStringList sqlEmptyDatabaseQuery() const;
};

//------------------------------------------------------

} // namespace DatabaseTest::Persistence::Database